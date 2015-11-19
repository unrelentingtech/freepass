extern crate libc;
extern crate secstr;
extern crate rusterpassword_capi;
extern crate freepass_core;

pub use rusterpassword_capi::*;

use std::{ptr,fs};
use std::collections::btree_map::Keys;
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
pub extern fn freepass_new_vault() -> *mut Vault {
    return Box::into_raw(Box::new(Vault::new()))
}

#[no_mangle]
pub extern fn freepass_vault_get_entry_names_iterator<'a>(vault_c: *const Vault) -> *mut Keys<'a, String, EncryptedEntry> {
    let vault = unsafe { assert!(!vault_c.is_null()); &*vault_c };
    Box::into_raw(Box::new(vault.entry_names()))
}

#[no_mangle]
pub unsafe extern fn freepass_entry_names_iterator_next<'a>(iter_c: *mut Keys<'a, String, EncryptedEntry>) -> *mut c_char {
    let iter = { assert!(!iter_c.is_null()); &mut *iter_c };
    match iter.next() {
        Some(s) => CString::new(s.clone()).unwrap().into_raw(),
        None => ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern fn freepass_free_entry_name(name_c: *mut c_char) {
    CString::from_raw(name_c);
}

#[no_mangle]
pub unsafe extern fn freepass_free_entry_names_iterator<'a>(iter_c: *mut Keys<'a, String, EncryptedEntry>) {
    Box::from_raw(iter_c);
}

#[no_mangle]
pub unsafe extern fn freepass_close_vault(vault_c: *mut Vault) {
    Box::from_raw(vault_c);
}
