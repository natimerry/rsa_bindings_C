extern crate alloc;
extern crate core;

use alloc::string::ToString;
use alloc::vec::Vec;
use core::str::from_utf8;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1::{DecodeRsaPublicKey, DecodeRsaPrivateKey, EncodeRsaPrivateKey, EncodeRsaPublicKey, LineEnding};
use std::{
    collections::HashMap,
    ffi::{c_char, CStr, CString},
    fmt::{Debug, Display}, ptr::null,
};
use std::ptr::null_mut;
use rsa::traits::PublicKeyParts;

#[repr(C)]
struct Buffer{
    data: *mut u8
}
#[macro_export]
macro_rules! c_str {
    ($lit:ident) => {
        std::ffi::CString::new($lit).unwrap().into_raw()
    };
}

fn r_str(ptr: *const c_char) -> String {
    if ptr == std::ptr::null() {
        "empty".to_string()
    } else {
        unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap().to_string() }
    }
}

#[no_mangle]
pub extern "C"
fn generate_pub_keys() {}


#[doc = "Returns PEM format string with the public key. Length of string = 1679"]
#[no_mangle]
pub extern "C"
fn generate_priv_keys(bits: u32) -> *mut c_char {
    let mut rng = rand::thread_rng();
    let priv_key = RsaPrivateKey::new(&mut rng, bits.try_into().unwrap())
        .expect("Unable to generate private keys");

    let pem_string = priv_key.to_pkcs1_pem(LineEnding::LF).unwrap().to_string();
    println!("Len of PrivateKey= {}", pem_string.len());
    c_str!(pem_string)
}

#[doc = "Returns PEM format string with the public key. Length of string = 426"]
#[no_mangle]
pub extern "C"
fn generate_public_key(rsa_private_key: *const c_char) -> *mut c_char {
    unsafe {
        let private_pem_string = r_str(rsa_private_key);
        let private_key = RsaPrivateKey::from_pkcs1_pem(&private_pem_string)
            .expect("Unable to generate the private key from the provided PEM string");

        match private_key.validate() {
            Ok(()) => (),
            Err(_) => return null_mut(),
        }
        let public_key = RsaPublicKey::from(private_key);
        let pem_string = public_key.to_pkcs1_pem(LineEnding::LF).unwrap();
        println!("Len of PublicKey = {}", pem_string.len());
        c_str!(pem_string)
    }
}

#[doc = "Returns a 256 byte long byte array containing the encrypted string"]
#[no_mangle]
pub extern "C"
fn encrypt_string(pem_public_key: *mut c_char, data: *mut c_char) -> Buffer {
    let pem_public_key = r_str(pem_public_key);
    let binding = r_str(data);
    let data = binding.as_bytes();
    let public_key = RsaPublicKey::from_pkcs1_pem(&pem_public_key).unwrap();
    let mut rng = rand::thread_rng();

    let mut enc_data = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).unwrap().into_boxed_slice();
    let data = enc_data.as_mut_ptr();
    println!("{:?}",enc_data);
    std::mem::forget(enc_data);
    Buffer{data}
}


#[doc = "Returns a c_string with the unencrypted data"]
#[no_mangle]
pub extern "C"
fn decrypt_string(pem_private_key: *mut c_char, data: *mut u8) -> *mut c_char {
    let pem_private_key = r_str(pem_private_key);
    let data = unsafe { Vec::from_raw_parts(data, 256, 256) };


    let private_key = RsaPrivateKey::from_pkcs1_pem(&pem_private_key).unwrap();
    private_key.validate().expect("Invalid PrivateKey");
    let enc_data = private_key.decrypt(Pkcs1v15Encrypt, &data).unwrap();

    let data = String::from_utf8(enc_data).unwrap();
    dbg!(&data);
    c_str!(data)
}
#[doc="Only use this if you HAVE to. This seems to cause a double free in my usage"]
#[no_mangle]
extern "C" fn free_buf(buf: Buffer) {
    let s = unsafe { std::slice::from_raw_parts_mut(buf.data,256 as usize) };
    let s = s.as_mut_ptr();
    unsafe {
        Box::from_raw(s);
    }
}