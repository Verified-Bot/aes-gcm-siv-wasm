use wasm_bindgen::prelude::*;
use aes_gcm_siv::{aead::{Aead, NewAead}, Aes256GcmSiv, Nonce};
use std::str;

#[wasm_bindgen]
pub fn encrypt(string: &str, key: &str) -> Vec<u8> {
    let key_bytes: &[u8] = &(base64::decode(key.as_bytes()).unwrap());
    let str_bytes = string.as_bytes();
    return aes_encrypt(str_bytes, &key_bytes);
}

#[wasm_bindgen]
pub fn decrypt(bytes: &[u8], key: &str) -> String {
    let key_bytes: &[u8] = &(base64::decode(key.as_bytes()).unwrap());
    return str::from_utf8(&aes_decrypt(&*bytes, &key_bytes).unwrap()).unwrap().to_string();
}

pub fn aes_encrypt(msg: &[u8], key: &[u8]) -> Vec<u8> {
    let aes_key = aes_gcm_siv::Key::from_slice(&key[..]);
    let cipher = Aes256GcmSiv::new(aes_key);
    let nonce = Nonce::from_slice(&[0; 12]);
    cipher.encrypt(nonce, msg).unwrap()
}

pub fn aes_decrypt(ciphertext: &[u8], key: &[u8]) -> Result<Vec<u8>, ()> {
    let aes_key = aes_gcm_siv::Key::from_slice(&key[..]);
    let cipher = Aes256GcmSiv::new(aes_key);
    let nonce = Nonce::from_slice(&[0; 12]);
    cipher.decrypt(nonce, ciphertext).map_err(|_| ())
}