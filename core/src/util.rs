extern crate libsodium_sys;
use libc::size_t;
use data::{StoredUsage, DerivedUsage, PasswordTemplate, Ed25519Usage};

pub fn blake2b(key: &[u8], msg: &[u8], len: usize) -> Vec<u8> {
    let mut dst = Vec::<u8>::with_capacity(len);
    unsafe {
        libsodium_sys::crypto_generichash_blake2b(
            dst.as_mut_ptr() as *mut u8, len,
            msg.as_ptr(), msg.len() as u64,
            key.as_ptr() as *const u8,
            key.len() as size_t);
        dst.set_len(len);
    }
    dst
}

pub fn guess_usage_stored(field_name: &str) -> StoredUsage {
    let fname = field_name.to_lowercase();
    if fname.contains("pass") || fname.contains("pin") || fname.contains("code") {
        StoredUsage::Password
    } else {
        StoredUsage::Text
    }
}

pub fn guess_usage_derived(field_name: &str) -> DerivedUsage {
    let fname = field_name.to_lowercase();
    if fname.contains("sign") {
        DerivedUsage::Ed25519Key(Ed25519Usage::Signify)
    } else if fname.contains("key") || fname.contains("ssh") {
        DerivedUsage::Ed25519Key(Ed25519Usage::SSH)
    } else {
        DerivedUsage::Password(PasswordTemplate::Maximum)
    }
}
