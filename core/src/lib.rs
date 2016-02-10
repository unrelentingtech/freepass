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


pub mod util;
pub mod result;
pub mod data;
pub mod output;
pub mod merge;

pub fn init() {
    sodiumoxide::init();
}
