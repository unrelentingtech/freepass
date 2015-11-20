extern crate libsodium_sys;
use libc::size_t;

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
