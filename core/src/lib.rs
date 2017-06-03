extern crate libc;

#[macro_use]
extern crate serde_derive;
extern crate serde_bytes;
extern crate serde_cbor;
extern crate serde;
extern crate base64;
extern crate byteorder;
#[cfg(all(unix, not(target_os = "android"), not(target_os = "ios")))]
extern crate unix_socket;

extern crate secstr;
extern crate rusterpassword;
extern crate sodiumoxide;
extern crate rand;

extern crate chrono;
#[cfg(feature = "keepass")]
extern crate keepass;

extern crate time;
#[cfg(feature = "filesystem")]
extern crate fuse;

pub mod util;
pub mod result;
pub mod data;
pub mod attachments;
pub mod vault;
pub mod encvault;
pub mod output;
pub mod merge;
pub mod import;

pub fn init() {
    sodiumoxide::init();
}
