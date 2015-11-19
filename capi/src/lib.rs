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

macro_rules! from_c {
    ( obj $o:expr ) => { unsafe { assert!(!$o.is_null()); &*$o }; };
    ( mut obj $o:expr ) => { { assert!(!$o.is_null()); &mut *$o } };
    ( cstr $s:expr ) => { unsafe { assert!(!$s.is_null()); CStr::from_ptr($s) }.to_str().unwrap() };
}

macro_rules! to_c {
    ( obj $o:expr ) => { Box::into_raw(Box::new($o)) };
    ( cstr $s:expr ) => { CString::new($s).unwrap().into_raw() };
}

#[no_mangle]
pub extern fn freepass_init() {
    freepass_core::init();
}

#[no_mangle]
pub extern fn freepass_gen_outer_key(master_key_c: *const SecStr) -> *mut SecStr {
    to_c![obj gen_outer_key(from_c![obj master_key_c])]
}

#[no_mangle]
pub extern fn freepass_gen_entries_key(master_key_c: *const SecStr) -> *mut SecStr {
    to_c![obj gen_entries_key(from_c![obj master_key_c])]
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
    let file_path = from_c![cstr file_path_c];
    let outer_key = from_c![obj outer_key_c];
    if let Ok(file) = fs::OpenOptions::new().read(true).write(true).open(&file_path) {
        if let Ok(vault) = Vault::open(outer_key, file) {
            return to_c![obj vault]
        }
    }
    return ptr::null_mut()
}

#[no_mangle]
pub extern fn freepass_new_vault() -> *mut Vault {
    to_c![obj Vault::new()]
}

#[no_mangle]
pub extern fn freepass_vault_get_entry_names_iterator<'a>(vault_c: *const Vault) -> *mut Keys<'a, String, EncryptedEntry> {
    to_c![obj from_c![obj vault_c].entry_names()]
}

#[no_mangle]
pub unsafe extern fn freepass_entry_names_iterator_next<'a>(iter_c: *mut Keys<'a, String, EncryptedEntry>) -> *mut c_char {
    match from_c![mut obj iter_c].next() {
        Some(s) => to_c![cstr s.clone()],
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
