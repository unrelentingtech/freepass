use std::iter::Iterator;
use result::Result;
use data::{Entry, EntryMetadata};

pub trait Vault {
    fn len(&self) -> usize;
    fn entry_names<'a>(&'a self) -> Box<Iterator<Item=&'a String> + 'a>;
    fn get_entry(&self, name: &str) -> Result<(Entry, EntryMetadata)>;
}

pub trait WritableVault {
    fn put_entry(&mut self, name: &str, entry: &Entry, metadata: &mut EntryMetadata) -> Result<()>;
    fn remove_entry(&mut self, name: &str);
}
