use secstr::SecStr;
use chrono::{DateTime, UTC};
use std::collections::btree_map::BTreeMap;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct EntryMetadata {
    #[serde(with = "serde_date_freepass", default = "UTC::now")]
    pub created_at: DateTime<UTC>,
    #[serde(with = "serde_date_freepass", default = "UTC::now")]
    pub updated_at: DateTime<UTC>,
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Default for EntryMetadata {
    fn default() -> EntryMetadata {
        EntryMetadata {
            created_at: UTC::now(),
            updated_at: UTC::now(),
            tags: Vec::new(),
        }
    }
}

#[derive(PartialEq, Clone, Default, Debug, Serialize, Deserialize, RustcDecodable)]
pub struct Entry {
    pub fields: BTreeMap<String, Field>
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, RustcDecodable)]
pub enum Field {
    Derived { counter: u32, site_name: Option<String>, usage: DerivedUsage },
    Stored { data: SecStr, usage: StoredUsage }
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize, RustcDecodable)]
pub enum DerivedUsage {
    Password(PasswordTemplate),
    Ed25519Key(Ed25519Usage),
    RawKey,
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize, RustcDecodable)]
pub enum Ed25519Usage {
    SSH,
    Signify,
    SQRL,
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize, RustcDecodable)]
pub enum StoredUsage {
    Text,
    Password,
    Attachments,
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize, RustcDecodable)]
pub enum PasswordTemplate {
    // Same numbers as in the rusterpassword C API
    Maximum = 60,
    Long    = 50,
    Medium  = 40,
    Short   = 30,
    Basic   = 20,
    Pin     = 10,
}

/// Decoding that ignores the old DateTime serialization (which was the whole chrono object structure)
/// to not fail when reading old files.
mod serde_date_freepass {
    use chrono::{DateTime, UTC, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};
    use serde_cbor::Value;

    pub fn serialize<S>(date: &DateTime<UTC>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(&date.to_rfc3339())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<UTC>, D::Error> where D: Deserializer<'de> {
        let v = try!(Value::deserialize(deserializer));
        match v {
            Value::String(s) => s.parse().map_err(serde::de::Error::custom),
            _ => Ok(UTC::now()),
        }
    }
}
