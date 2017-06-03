use std::{io, str};
use std::collections::btree_map::BTreeMap;
use secstr::SecStr;
#[cfg(feature = "keepass")] use keepass::{Database, Node, Value};
use vault::Vault;
use result::*;
use data::*;
use util;

#[derive(Debug, Default, Clone)]
pub struct ImportVault {
    entries: BTreeMap<String, (Entry, EntryMetadata)>,
}

impl Vault for ImportVault {
    fn len(&self) -> usize {
        self.entries.len()
    }

    fn entry_names<'a>(&'a self) -> Box<Iterator<Item=&'a String> + 'a> {
        Box::new(self.entries.keys())
    }

    fn get_entry(&self, name: &str) -> Result<(Entry, EntryMetadata)> {
        self.entries.get(name).map(|x| x.to_owned()).ok_or(Error::EntryNotFound)
    }
}

// Wow, keepass's format is even less structured than ours (even the title is just a "Title" field)
// Also the keepass library uses rust-crypto, which means the import will be very slow in a debug build
#[cfg(feature = "keepass")]
pub fn kdbx<T: io::Read>(source: &mut T, password: &SecStr) -> Result<ImportVault> {
    let db = try!(Database::open(source, try!(str::from_utf8(password.unsecure()))));
    let mut vault = ImportVault::default();
    for node in &db.root {
        match node {
            Node::Group(_) => { },
            Node::Entry(kentry) => {
                let mut entry = Entry::default();
                for (k, v) in kentry.fields.iter().filter(|x| x.0 != "Title") {
                    let data = match *v {
                        Value::Unprotected(ref s) => SecStr::from(s.to_owned()),
                        Value::Protected(ref s) => s.to_owned(),
                    };
                    entry.fields.insert(k.to_owned(), Field::Stored { data: data, usage: util::guess_usage_stored(k) });
                }
                vault.entries.insert(kentry.get_title().unwrap_or("??? Untitled imported entry").to_owned(), (entry, EntryMetadata::default()));
            }
        }
    }
    Ok(vault)
}
