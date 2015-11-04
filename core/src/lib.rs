extern crate cbor;
extern crate rustc_serialize;
extern crate secstr;
extern crate rusterpassword;
extern crate libc;
extern crate libsodium_sys as sodium;
extern crate sodiumoxide;
extern crate chrono;

use secstr::SecStr;
use rusterpassword::gen_site_seed;
use chrono::{DateTime, UTC};
use cbor::{Encoder, Decoder, CborError};
use libc::size_t;
use std::collections::btree_map::{BTreeMap, Keys};
use sodiumoxide::crypto::secretbox::xsalsa20poly1305 as secbox;


#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Vault {
    pub version: u16,
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

#[derive(PartialEq, Debug, RustcDecodable, RustcEncodable)]
pub enum Field {
    Derived { counter: u32, site_name: Option<String>, usage: DerivedUsage },
    Stored { data: SecStr, usage: StoredUsage }
}

#[derive(PartialEq, Debug, RustcDecodable, RustcEncodable)]
pub enum DerivedUsage {
    Password(PasswordTemplate)
}

#[derive(PartialEq, Debug, RustcDecodable, RustcEncodable)]
pub enum StoredUsage {
    Password
}

#[derive(PartialEq, Debug, RustcDecodable, RustcEncodable)]
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
pub enum EntryError {
    WrongEntriesKeyLength,
    WrongNonceLength,
    SeedGenerationError,
    DecryptionError,
    CodecError(CborError),
    DataError,
    EntryNotFound,
}

impl From<CborError> for EntryError {
    fn from(err: CborError) -> EntryError {
        EntryError::CodecError(err)
    }
}

pub type EntryResult<T> = Result<T, EntryError>;

impl Vault {

    pub fn entry_names(&self) -> Keys<String, EncryptedEntry> {
        self.entries.keys()
    }

    pub fn get_entry(&self, entries_key: &SecStr, name: &str) -> EntryResult<(Entry, EntryMetadata)> {
        if let Some(ee) = self.entries.get(name) {
            let nonce_wrapped = try!(secbox::Nonce::from_slice(&ee.nonce).ok_or(EntryError::WrongNonceLength));
            let entry_key_wrapped = try!(gen_entry_key(entries_key, name, ee.counter));
            let plaintext = SecStr::new(try!(secbox::open(&ee.ciphertext, &nonce_wrapped, &entry_key_wrapped).map_err(|_| EntryError::DecryptionError)));
            let entry = try!(try!(Decoder::from_bytes(plaintext.unsecure()).decode::<Entry>().next().ok_or(EntryError::DataError)));
            Ok((entry, ee.metadata.clone()))
        } else {
            Err(EntryError::EntryNotFound)
        }
    }

    pub fn put_entry(&mut self, entries_key: &SecStr, name: &str, entry: &Entry, metadata: &mut EntryMetadata) -> EntryResult<()> {
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

}

fn gen_entry_key(entries_key: &SecStr, name: &str, counter: u32) -> EntryResult<secbox::Key> {
    let entry_key = try!(gen_site_seed(entries_key, name, counter).map_err(|_| EntryError::SeedGenerationError));
    Ok(try!(secbox::Key::from_slice(entry_key.unsecure()).ok_or(EntryError::WrongEntriesKeyLength)))
}

pub fn gen_entries_key(master_key: &SecStr) -> SecStr {
    let mut msg = vec![];
    msg.extend(b"technology.unrelenting.freepass");
    let mut dst = Vec::<u8>::with_capacity(64);
    unsafe {
        sodium::crypto_generichash_blake2b(
            dst.as_mut_ptr() as *mut u8, 64,
            msg.as_ptr(), msg.len() as u64,
            master_key.unsecure().as_ptr() as *const u8,
            master_key.unsecure().len() as size_t);
        dst.set_len(64);
    }
    SecStr::new(dst)
}

#[cfg(test)]
mod tests {
    use super::*;
    use secstr::*;
    use rusterpassword::*;
    use std::collections::btree_map::BTreeMap;

    #[test]
    fn test_roundtrip_entry() {
        let mut fs = BTreeMap::new();
        fs.insert("password".to_owned(), Field::Derived { counter: 4, site_name: Some("twitter.com".to_owned()), usage: DerivedUsage::Password(PasswordTemplate::Maximum) });
        fs.insert("old_password".to_owned(), Field::Stored { data: SecStr::from("h0rse"), usage: StoredUsage::Password });
        let twitter = Entry { fields: fs };
        let mut metadata = EntryMetadata::new();
        let mut vault = Vault { version: 0, padding: b"".to_vec(), entries: BTreeMap::new() };
        let master_key = gen_master_key(SecStr::from("Correct Horse Battery Staple"), "Clarke Griffin").unwrap();
        let entries_key = gen_entries_key(&master_key);
        vault.put_entry(&entries_key, "twitter", &twitter, &mut metadata).unwrap();
        assert!(vault.get_entry(&entries_key, "twitter").unwrap() == (twitter, metadata));
    }
}
