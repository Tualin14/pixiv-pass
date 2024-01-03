use aes_gcm::{AeadCore, Aes256Gcm, Key, KeyInit};
use aes_gcm::aead::{Aead, OsRng};
use base64::{Engine as _, engine::general_purpose};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn encrypt(key_str: &str, plain_text_str: &str) -> JsValue {
    return match _encrypt(key_str, plain_text_str) {
        Ok(x) => JsValue::from_str(x.as_str()),
        Err(err) => JsValue::from_str(err),
    };
}

fn _encrypt(key_str: &str, plain_text_str: &str) -> Result<String, &'static str> {
    let mut key = key_str.as_bytes().to_vec();
    if key.len() < 32 {
        key.resize(32, 0);
    }
    let key_slice: &[u8] = &key[..32];

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_slice));

    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let encrypt_bytes = match cipher.encrypt(&nonce, plain_text_str.as_bytes()) {
        Ok(bytes) => bytes,
        Err(_) => return Err("Encryption error"),
    };
    let mut cipher_text_bytes = Vec::new();
    cipher_text_bytes.extend_from_slice(&nonce.to_vec());
    cipher_text_bytes.extend(encrypt_bytes);

    let cipher_text = general_purpose::URL_SAFE_NO_PAD.encode(cipher_text_bytes);
    Ok(cipher_text)
}
