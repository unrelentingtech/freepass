extern crate cbor;
extern crate rustc_serialize;
extern crate secstr;
extern crate rusterpassword;
extern crate libc;
extern crate libsodium_sys as sodium;
extern crate sodiumoxide;

use secstr::*;
use rusterpassword::*;
use cbor::{Encoder, Decoder};
use libc::size_t;
use std::collections::btree_map::{BTreeMap, Keys};
use sodiumoxide::crypto::secretbox::xsalsa20poly1305 as secbox;

macro_rules! try_custom {
    ($e:expr, $err:expr) => (match $e { Ok(e) => e, Err(_) => return Err($err) })
}

macro_rules! try_custom_option {
    ($e:expr, $err:expr) => (match $e { Some(e) => e, None => return Err($err) })
}


#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Vault {
    version: u16,
    padding: Vec<u8>,
    entries: BTreeMap<String, (Vec<u8>, u32, Vec<u8>)> // mapping: name → nonce, counter, encrypted Entry
}

#[derive(PartialEq, Debug, RustcDecodable, RustcEncodable)]
pub struct Entry {
    fields: BTreeMap<String, Field>
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
    Maximum, Long, Medium, Short, Basic, Pin
}

#[derive(Debug)]
pub enum EntryError {
    WrongEntriesKeyLength,
    WrongNonceLength,
    SeedGenerationError,
    DecryptionError,
    EncodingError,
    DecodingError,
    EntryNotFound
}

impl Vault {
    pub fn entry_names(&self) -> Keys<String, (Vec<u8>, u32, Vec<u8>)> {
        self.entries.keys()
    }

    pub fn get_entry(&self, entries_key: &SecStr, name: &str) -> Result<Entry, EntryError> {
        if let Some(&(ref nonce, counter, ref ciphertext)) = self.entries.get(name) {
            let nonce_wrapped = try_custom_option!(secbox::Nonce::from_slice(&nonce), EntryError::WrongNonceLength);
            let entry_key_wrapped = try!(gen_entry_key(entries_key, name, counter));
            let plaintext = SecStr::new(try_custom!(secbox::open(&ciphertext, &nonce_wrapped, &entry_key_wrapped), EntryError::DecryptionError));
            Ok(try_custom!(try_custom_option!(Decoder::from_bytes(plaintext.unsecure()).decode::<Entry>().next(), EntryError::DecodingError), EntryError::DecodingError))
        } else {
            Err(EntryError::EntryNotFound)
        }
    }

    pub fn put_entry(&mut self, entries_key: &SecStr, name: &str, entry: &Entry) -> Result<(), EntryError> {
        let counter = match self.entries.get(name) {
            Some(&(_, counter, _)) => counter + 1,
            _ => 1
        };
        let nonce_wrapped = secbox::gen_nonce();
        let secbox::Nonce(nonce) = nonce_wrapped;
        let entry_key_wrapped = try!(gen_entry_key(entries_key, name, counter));
        let mut e = Encoder::from_memory();
        try_custom!(e.encode(&[entry]), EntryError::EncodingError);
        let plaintext = SecStr::new(e.into_bytes());
        let ciphertext = secbox::seal(plaintext.unsecure(), &nonce_wrapped, &entry_key_wrapped);
        self.entries.insert(String::from(name), (nonce.to_vec(), counter, ciphertext));
        Ok(())
    }

}

fn gen_entry_key(entries_key: &SecStr, name: &str, counter: u32) -> Result<secbox::Key, EntryError> {
    let entry_key = try_custom!(gen_site_seed(entries_key, name, counter), EntryError::SeedGenerationError);
    Ok(try_custom_option!(secbox::Key::from_slice(entry_key.unsecure()), EntryError::WrongEntriesKeyLength))
}

pub fn gen_entries_key(master_key: &SecStr) -> SecStr {
    let mut msg = vec![];
    msg.extend(b"technology.unrelenting.freepass");
    let mut dst = Vec::<u8>::with_capacity(64);
    unsafe {
        sodium::crypto_generichash_blake2b(
            dst.as_mut_ptr() as *mut u8, 64,
            msg.as_ptr(), msg.len() as size_t,
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
    fn test_roundtrip() {
        let mut fs = BTreeMap::new();
        fs.insert(String::from("password"), Field::Derived { counter: 4, site_name: Some(String::from("twitter.com")), usage: DerivedUsage::Password(PasswordTemplate::Maximum) });
        fs.insert(String::from("old_password"), Field::Stored { data: SecStr::from("h0rse"), usage: StoredUsage::Password });
        let twitter = Entry { fields: fs };
        let mut vault = Vault { version: 0, padding: b"".to_vec(), entries: BTreeMap::new() };
        let master_key = gen_master_key(SecStr::from("Correct Horse Battery Staple"), "Clarke Griffin").unwrap();
        let entries_key = gen_entries_key(&master_key);
        vault.put_entry(&entries_key, "twitter", &twitter).unwrap();
        assert!(vault.get_entry(&entries_key, "twitter").unwrap() == twitter);
    }
}
