extern crate libc;
extern crate secstr;
extern crate rusterpassword_capi;
extern crate freepass_core;

pub use rusterpassword_capi::*;

use std::{ptr,fs};
use std::ffi::*;
use libc::*;
use secstr::*;
use freepass_core::data::*;
// use freepass_core::output::*;

#[no_mangle]
pub extern fn freepass_init() {
    freepass_core::init();
}

#[no_mangle]
pub extern fn freepass_gen_outer_key(master_key_c: *const SecStr) -> *mut SecStr {
    let master_key = unsafe { assert!(!master_key_c.is_null()); &*master_key_c };
    Box::into_raw(Box::new(gen_outer_key(master_key)))
}

#[no_mangle]
pub extern fn freepass_gen_entries_key(master_key_c: *const SecStr) -> *mut SecStr {
    let master_key = unsafe { assert!(!master_key_c.is_null()); &*master_key_c };
    Box::into_raw(Box::new(gen_entries_key(master_key)))
}

#[no_mangle]
pub unsafe extern fn freepass_free_outer_key(outer_key_c: *mut SecStr) {
    Box::from_raw(outer_key_c);
}

#[no_mangle]
pub unsafe extern fn freepass_free_entries_key(entries_key_c: *mut SecStr) {
    Box::from_raw(entries_key_c);
}

#[no_mangle]
pub extern fn freepass_open_vault(file_path_c: *const c_char, outer_key_c: *const SecStr) -> *mut Vault {
    let file_path = unsafe { assert!(!file_path_c.is_null()); CStr::from_ptr(file_path_c) }.to_str().unwrap();
    let outer_key = unsafe { assert!(!outer_key_c.is_null()); &*outer_key_c };
    if let Ok(file) = fs::OpenOptions::new().read(true).write(true).open(&file_path) {
        if let Ok(vault) = Vault::open(outer_key, file) {
            return Box::into_raw(Box::new(vault))
        }
    }
    return ptr::null_mut()
}

#[no_mangle]
pub unsafe extern fn freepass_close_vault(vault: *mut Vault) {
    Box::from_raw(vault);
}
