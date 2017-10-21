use std::io;
use std::boxed::Box;
use std::collections::btree_map::BTreeMap;
use rand::Rng;
use rand::os::OsRng;
use sodiumoxide::crypto::secretbox::xsalsa20poly1305 as secbox;
use crypto::aes;
use chrono::UTC;
use serde_cbor;
use serde_bytes;
use secstr::SecStr;
use rusterpassword::gen_site_seed;
use result::{Error, Result};
use vault::{Vault, WritableVault};
use data::*;
use util::blake2b;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct EncryptedEntry {
    #[serde(with = "serde_bytes")]
    pub nonce: Vec<u8>,
    pub counter: u32,
    #[serde(with = "serde_bytes")]
    pub ciphertext: Vec<u8>,
    pub metadata: EntryMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedVault {
    pub version: u16,
    #[serde(with = "serde_bytes")]
    pub nonce: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub ciphertext: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecryptedVaultData {
    #[serde(with = "serde_bytes")]
    pub padding: Vec<u8>,
    pub entries: BTreeMap<String, EncryptedEntry>,
}

pub struct DecryptedVault {
    pub data: DecryptedVaultData,
    entries_key: SecStr,
    outer_key: SecStr,
}

impl DecryptedVault {
    /// Returns the decrypted entry without decoding CBOR.
    pub fn get_entry_cbor(&self, name: &str) -> Result<(Vec<u8>, EntryMetadata)> {
        if let Some(ee) = self.data.entries.get(name) {
            let nonce_wrapped = secbox::Nonce::from_slice(&ee.nonce)
                .ok_or(Error::WrongEntryNonceLength)?;
            let entry_key_wrapped = gen_entry_key(&self.entries_key, name, ee.counter)?;
            let plainbytes = secbox::open(&ee.ciphertext, &nonce_wrapped, &entry_key_wrapped)
                .map_err(|_| Error::DecryptionError)?;
            Ok((plainbytes, ee.metadata.clone()))
        } else {
            Err(Error::EntryNotFound)
        }
    }
}

impl Vault for DecryptedVault {
    fn len(&self) -> usize {
        self.data.entries.len()
    }

    fn entry_names<'a>(&'a self) -> Box<Iterator<Item = &'a String> + 'a> {
        Box::new(self.data.entries.keys())
    }

    fn get_entry(&self, name: &str) -> Result<(Entry, EntryMetadata)> {
        let (plainbytes, metadata) = self.get_entry_cbor(name)?;
        let plaintext = SecStr::new(plainbytes); // For zeroing out CBOR bytes (on Drop) after decoding it
        let entry = serde_cbor::from_slice(plaintext.unsecure())?;
        Ok((entry, metadata))
    }
}

impl WritableVault for DecryptedVault {
    fn put_entry(&mut self, name: &str, entry: &Entry, metadata: &mut EntryMetadata) -> Result<()> {
        let counter = self.data
            .entries
            .get(name)
            .map(|ee| ee.counter + 1)
            .unwrap_or(1);
        let nonce_wrapped = secbox::gen_nonce();
        let secbox::Nonce(nonce) = nonce_wrapped;
        let entry_key_wrapped = gen_entry_key(&self.entries_key, name, counter)?;
        let plaintext = SecStr::new(serde_cbor::to_vec(&entry)?);
        let ciphertext = secbox::seal(plaintext.unsecure(), &nonce_wrapped, &entry_key_wrapped);
        metadata.updated_at = UTC::now();
        self.data.entries.insert(
            name.to_owned(),
            EncryptedEntry {
                nonce: nonce.to_vec(),
                counter: counter,
                ciphertext: ciphertext,
                metadata: metadata.clone(),
            },
        );
        Ok(())
    }

    fn remove_entry(&mut self, name: &str) {
        self.data.entries.remove(name);
    }
}

impl DecryptedVault {
    pub fn new(entries_key: SecStr, outer_key: SecStr) -> DecryptedVault {
        DecryptedVault {
            data: DecryptedVaultData { padding: b"".to_vec(), entries: BTreeMap::new() },
            entries_key: entries_key,
            outer_key: outer_key,
        }
    }

    pub fn open<T: io::Read>(entries_key: SecStr, outer_key: SecStr, reader: T) -> Result<DecryptedVault> {
        let wrapper: EncryptedVault = serde_cbor::from_reader(reader)?;
        let mut plaintext = SecStr::new(vec![0; wrapper.ciphertext.len()]);
        aes::ctr(aes::KeySize::KeySize128, outer_key.unsecure(), &wrapper.nonce)
            .process(&wrapper.ciphertext, &mut plaintext.unsecure_mut());
        let data = serde_cbor::from_slice(plaintext.unsecure())?;
        Ok(DecryptedVault { data: data, entries_key: entries_key, outer_key: outer_key })
    }

    pub fn save<T: io::Write>(&mut self, mut writer: T) -> Result<()> {
        let mut plaintext = SecStr::new(serde_cbor::to_vec(&self.data)?);
        let mut rng = OsRng::new()?;
        let padding_size = rng.gen_range(0, 1024 * 10);
        self.data.padding = vec![0; padding_size];
        rng.fill_bytes(&mut self.data.padding);
        let mut iv = vec![0; 16];
        rng.fill_bytes(&mut iv);
        let mut ciphertext = vec![0; plaintext.unsecure().len()];
        aes::ctr(aes::KeySize::KeySize128, self.outer_key.unsecure(), &iv)
            .process(&plaintext.unsecure_mut(), &mut ciphertext);
        let wrapper = EncryptedVault { version: 0, nonce: iv, ciphertext: ciphertext };
        serde_cbor::ser::to_writer(&mut writer, &wrapper)?;
        Ok(())
    }
}

fn gen_entry_key(entries_key: &SecStr, name: &str, counter: u32) -> Result<secbox::Key> {
    let entry_key = gen_site_seed(entries_key, name, counter).map_err(|_| {
            Error::SeedGenerationError
        })?;
    Ok(
        secbox::Key::from_slice(entry_key.unsecure())
            .ok_or(Error::WrongEntriesKeyLength)?,
    )
}

pub fn gen_outer_key(master_key: &SecStr) -> SecStr {
    SecStr::new(blake2b(master_key.unsecure(), b"freepass.outer", 16))
}

pub fn gen_entries_key(master_key: &SecStr) -> SecStr {
    SecStr::new(blake2b(master_key.unsecure(), b"freepass.entries", 64))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusterpassword::*;

    fn example_entry() -> Entry {
        let mut twitter = Entry::default();
        twitter.fields.insert(
            "password".to_owned(),
            Field::Derived {
                counter: 4,
                site_name: Some("twitter.com".to_owned()),
                usage: DerivedUsage::Password(PasswordTemplate::Maximum),
            },
        );
        twitter.fields.insert(
            "old_password".to_owned(),
            Field::Stored {
                data: SecStr::from("h0rse"),
                usage: StoredUsage::Password,
            },
        );
        twitter
    }

    #[test]
    fn test_roundtrip_entry() {
        let twitter = example_entry();
        let mut metadata = EntryMetadata::default();
        let master_key = gen_master_key(SecStr::from("Correct Horse Battery Staple"), "Clarke Griffin").unwrap();
        let mut vault = DecryptedVault::new(gen_entries_key(&master_key), gen_outer_key(&master_key));
        vault.put_entry("twitter", &twitter, &mut metadata).unwrap();
        assert!(vault.get_entry("twitter").unwrap() == (twitter, metadata));
    }

    #[test]
    fn test_roundtrip_vault() {
        let master_key = gen_master_key(SecStr::from("Correct Horse Battery Staple"), "Clarke Griffin").unwrap();
        let mut vault = DecryptedVault::new(gen_entries_key(&master_key), gen_outer_key(&master_key));
        vault.data.entries.insert(
            "test".to_owned(),
            EncryptedEntry {
                nonce: b"fuck".to_vec(),
                counter: 123,
                ciphertext: b"hello".to_vec(),
                metadata: EntryMetadata::default(),
            },
        );
        let mut storage = Vec::new();
        vault.save(&mut storage).unwrap();
        let loaded_vault = DecryptedVault::open(gen_entries_key(&master_key), gen_outer_key(&master_key), &storage[..]);
        assert!(loaded_vault.unwrap().data.entries == vault.data.entries);
    }
}
