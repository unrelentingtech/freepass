use secstr::SecStr;
use chrono::{DateTime, UTC};
use std::collections::btree_map::BTreeMap;

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

#[derive(PartialEq, Clone, Debug, RustcDecodable, RustcEncodable)]
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
