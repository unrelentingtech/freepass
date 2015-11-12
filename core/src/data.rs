extern crate libsodium_sys;

use secstr::SecStr;
use rusterpassword::gen_site_seed;
use chrono::{DateTime, UTC};
use cbor::{Encoder, Decoder, CborError};
use libc::size_t;
use rand::Rng;
use rand::os::OsRng;
use std::io;
use std::result;
use std::string;
use std::collections::btree_map::{BTreeMap, Keys};
use sodiumoxide::crypto::secretbox::xsalsa20poly1305 as secbox;
use sodiumoxide::crypto::stream::aes128ctr as outerstream;
use byteorder;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct EncryptedVault {
    pub version: u16,
    pub nonce: Vec<u8>,
    pub ciphertext: Vec<u8>,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Vault {
    pub padding: Vec<u8>,
    pub entries: BTreeMap<String, EncryptedEntry>,
}

#[derive(PartialEq, Debug, RustcDecodable, RustcEncodable)]
pub struct EncryptedEntry {
    pub nonce: Vec<u8>,
    pub counter: u32,
    pub ciphertext: Vec<u8>,
    pub metadata: EntryMetadata,
}

#[derive(PartialEq, Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct EntryMetadata {
    pub created_at: DateTime<UTC>,
    pub updated_at: DateTime<UTC>,
    pub tags: Vec<String>,
}

impl EntryMetadata {
    pub fn new() -> EntryMetadata {
        EntryMetadata {
            created_at: UTC::now(),
            updated_at: UTC::now(),
            tags: Vec::new(),
        }
    }
}

#[derive(PartialEq, Debug, RustcDecodable, RustcEncodable)]
pub struct Entry {
    pub fields: BTreeMap<String, Field>
}

impl Entry {
    pub fn new() -> Entry {
        Entry { fields: BTreeMap::new() }
    }
}

#[derive(PartialEq, Clone, Debug, RustcDecodable, RustcEncodable)]
pub enum Field {
    Derived { counter: u32, site_name: Option<String>, usage: DerivedUsage },
    Stored { data: SecStr, usage: StoredUsage }
}

#[derive(PartialEq, Clone, Copy, Debug, RustcDecodable, RustcEncodable)]
pub enum DerivedUsage {
    Password(PasswordTemplate),
    Ed25519Key(Ed25519Usage),
    RawKey,
}

#[derive(PartialEq, Clone, Copy, Debug, RustcDecodable, RustcEncodable)]
pub enum Ed25519Usage {
    SSH,
    Signify,
    SQRL,
}

#[derive(PartialEq, Clone, Copy, Debug, RustcDecodable, RustcEncodable)]
pub enum StoredUsage {
    Text,
    Password,
}

#[derive(PartialEq, Clone, Copy, Debug, RustcDecodable, RustcEncodable)]
pub enum PasswordTemplate {
    // Same numbers as in the rusterpassword C API
    Maximum = 60,
    Long    = 50,
    Medium  = 40,
    Short   = 30,
    Basic   = 20,
    Pin     = 10,
}

#[derive(Debug)]
pub enum Error {
    WrongEntriesKeyLength,
    WrongEntryNonceLength,
    WrongOuterNonceLength,
    WrongOuterKeyLength,
    WrongDerivedKeyLength,
    InappropriateFormat,
    SeedGenerationError,
    DecryptionError,
    CodecError(CborError),
    ByteCodecError(byteorder::Error),
    StringCodecError(string::FromUtf8Error),
    OtherError(io::Error),
    DataError,
    EntryNotFound,
    NotImplemented,
    NotAvailableOnPlatform,
    SSHAgentSocketNotFound,
}

impl From<CborError> for Error {
    fn from(err: CborError) -> Error {
        Error::CodecError(err)
    }
}

impl From<byteorder::Error> for Error {
    fn from(err: byteorder::Error) -> Error {
        Error::ByteCodecError(err)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Error {
        Error::StringCodecError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::OtherError(err)
    }
}

pub type Result<T> = result::Result<T, Error>;

impl Vault {

    pub fn new() -> Vault {
        Vault { padding: b"".to_vec(), entries: BTreeMap::new() }
    }

    pub fn entry_names(&self) -> Keys<String, EncryptedEntry> {
        self.entries.keys()
    }

    pub fn get_entry(&self, entries_key: &SecStr, name: &str) -> Result<(Entry, EntryMetadata)> {
        if let Some(ee) = self.entries.get(name) {
            let nonce_wrapped = try!(secbox::Nonce::from_slice(&ee.nonce).ok_or(Error::WrongEntryNonceLength));
            let entry_key_wrapped = try!(gen_entry_key(entries_key, name, ee.counter));
            let plaintext = SecStr::new(try!(secbox::open(&ee.ciphertext, &nonce_wrapped, &entry_key_wrapped).map_err(|_| Error::DecryptionError)));
            let entry = try!(try!(Decoder::from_bytes(plaintext.unsecure()).decode::<Entry>().next().ok_or(Error::DataError)));
            Ok((entry, ee.metadata.clone()))
        } else {
            Err(Error::EntryNotFound)
        }
    }

    pub fn put_entry(&mut self, entries_key: &SecStr, name: &str, entry: &Entry, metadata: &mut EntryMetadata) -> Result<()> {
        let counter = self.entries.get(name).map(|ee| ee.counter + 1).unwrap_or(1);
        let nonce_wrapped = secbox::gen_nonce();
        let secbox::Nonce(nonce) = nonce_wrapped;
        let entry_key_wrapped = try!(gen_entry_key(entries_key, name, counter));
        let mut e = Encoder::from_memory();
        try!(e.encode(&[&*entry]));
        let plaintext = SecStr::new(e.into_bytes());
        let ciphertext = secbox::seal(plaintext.unsecure(), &nonce_wrapped, &entry_key_wrapped);
        metadata.updated_at = UTC::now();
        self.entries.insert(name.to_owned(), EncryptedEntry {
            nonce: nonce.to_vec(), counter: counter, ciphertext: ciphertext, metadata: metadata.clone()
        });
        Ok(())
    }

    pub fn remove_entry(&mut self, name: &str) {
        self.entries.remove(name);
    }

    pub fn open<T: io::Read>(outer_key: &SecStr, reader: T) -> Result<Vault> {
        let wrapper = try!(try!(Decoder::from_reader(reader).decode::<EncryptedVault>().next().ok_or(Error::DataError)));
        let nonce_wrapped = try!(outerstream::Nonce::from_slice(&wrapper.nonce).ok_or(Error::WrongOuterNonceLength));
        let outer_key_wrapped = try!(outerstream::Key::from_slice(outer_key.unsecure()).ok_or(Error::WrongOuterKeyLength));
        let plaintext = SecStr::new(outerstream::stream_xor(&wrapper.ciphertext, &nonce_wrapped, &outer_key_wrapped));
        let vault = try!(try!(Decoder::from_bytes(plaintext.unsecure()).decode::<Vault>().next().ok_or(Error::DataError)));
        Ok(vault)
    }

    pub fn save<T: io::Write>(&mut self, outer_key: &SecStr, writer: T) -> Result<()> {
        let mut rng = try!(OsRng::new());
        let padding_size = rng.gen_range(0, 1024*10);
        self.padding = vec![0; padding_size];
        rng.fill_bytes(&mut self.padding);
        let nonce_wrapped = outerstream::gen_nonce();
        let outerstream::Nonce(nonce) = nonce_wrapped;
        let outer_key_wrapped = try!(outerstream::Key::from_slice(outer_key.unsecure()).ok_or(Error::WrongOuterKeyLength));
        let mut e = Encoder::from_memory();
        try!(e.encode(&[&*self]));
        let plaintext = SecStr::new(e.into_bytes());
        let ciphertext = outerstream::stream_xor(plaintext.unsecure(), &nonce_wrapped, &outer_key_wrapped);
        let wrapper = EncryptedVault { version: 0, nonce: nonce.to_vec(), ciphertext: ciphertext };
        let mut outer_e = Encoder::from_writer(writer);
        try!(outer_e.encode(&[&wrapper]));
        Ok(())
    }

}

fn gen_entry_key(entries_key: &SecStr, name: &str, counter: u32) -> Result<secbox::Key> {
    let entry_key = try!(gen_site_seed(entries_key, name, counter).map_err(|_| Error::SeedGenerationError));
    Ok(try!(secbox::Key::from_slice(entry_key.unsecure()).ok_or(Error::WrongEntriesKeyLength)))
}

pub fn gen_outer_key(master_key: &SecStr) -> SecStr {
    blake2b(master_key, b"freepass.outer".to_vec(), 16)
}

pub fn gen_entries_key(master_key: &SecStr) -> SecStr {
    blake2b(master_key, b"freepass.entries".to_vec(), 64)
}

fn blake2b(master_key: &SecStr, msg: Vec<u8>, len: usize) -> SecStr {
    let mut dst = Vec::<u8>::with_capacity(len);
    unsafe {
        libsodium_sys::crypto_generichash_blake2b(
            dst.as_mut_ptr() as *mut u8, len,
            msg.as_ptr(), msg.len() as u64,
            master_key.unsecure().as_ptr() as *const u8,
            master_key.unsecure().len() as size_t);
        dst.set_len(len);
    }
    SecStr::new(dst)
}

#[cfg(test)]
mod tests {
    use super::*;
    use secstr::*;
    use rusterpassword::*;

    fn example_entry() -> Entry {
        let mut twitter = Entry::new();
        twitter.fields.insert("password".to_owned(), Field::Derived {
            counter: 4, site_name: Some("twitter.com".to_owned()), usage: DerivedUsage::Password(PasswordTemplate::Maximum)
        });
        twitter.fields.insert("old_password".to_owned(), Field::Stored {
            data: SecStr::from("h0rse"), usage: StoredUsage::Password
        });
        twitter
    }

    #[test]
    fn test_roundtrip_entry() {
        let twitter = example_entry();
        let mut metadata = EntryMetadata::new();
        let master_key = gen_master_key(SecStr::from("Correct Horse Battery Staple"), "Clarke Griffin").unwrap();
        let entries_key = gen_entries_key(&master_key);
        let mut vault = Vault::new();
        vault.put_entry(&entries_key, "twitter", &twitter, &mut metadata).unwrap();
        assert!(vault.get_entry(&entries_key, "twitter").unwrap() == (twitter, metadata));
    }

    #[test]
    fn test_roundtrip_vault() {
        let master_key = gen_master_key(SecStr::from("Correct Horse Battery Staple"), "Clarke Griffin").unwrap();
        let outer_key = gen_outer_key(&master_key);
        let mut vault = Vault::new();
        vault.entries.insert("test".to_string(), EncryptedEntry {
            nonce: b"fuck".to_vec(), counter: 123, ciphertext: b"hello".to_vec(), metadata: EntryMetadata::new()
        });
        let mut storage = Vec::new();
        vault.save(&outer_key, &mut storage).unwrap();
        let loaded_vault = Vault::open(&outer_key, &storage[..]);
        assert!(loaded_vault.unwrap().entries == vault.entries);
    }
}
