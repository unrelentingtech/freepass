extern crate libc;

extern crate rustc_serialize;
extern crate cbor;
extern crate byteorder;
#[cfg(all(unix, not(target_os = "android"), not(target_os = "ios")))] extern crate unix_socket;

extern crate secstr;
extern crate rusterpassword;
extern crate sodiumoxide;
extern crate rand;

extern crate chrono;
#[cfg(feature = "keepass")] extern crate keepass;

#[cfg(feature = "filesystem")] extern crate time;
#[cfg(feature = "filesystem")] extern crate fuse;

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
