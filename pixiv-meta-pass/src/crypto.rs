use aes_gcm::{AeadCore, Aes256Gcm, Key, KeyInit, Nonce};
use aes_gcm::aead::{Aead, OsRng};
use base64::{Engine as _, engine::general_purpose};

pub fn _encrypt(key_str: &str, plain_text_str: &str) -> Result<String, &'static str> {
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


pub fn decrypt(key_str: &str, cipher_text_str: &str) -> Result<String, &'static str> {
    let cipher_bytes = match general_purpose::URL_SAFE_NO_PAD.decode(cipher_text_str) {
        Ok(bytes) => bytes,
        Err(_) => return Err("Decoding error"),
    };

    let nonce_bytes = &cipher_bytes[..12];
    let nonce = Nonce::from_slice(nonce_bytes);

    let mut key = key_str.as_bytes().to_vec();
    if key.len() < 32 {
        key.resize(32, 0);
    }
    let key_slice: &[u8] = &key[..32];

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_slice));

    let decrypted_bytes = match cipher.decrypt(nonce, &cipher_bytes[12..]) {
        Ok(bytes) => bytes,
        Err(_) => return Err("Decryption error"),
    };
    match String::from_utf8(decrypted_bytes) {
        Ok(decrypted_text) => Ok(decrypted_text),
        Err(_) => Err("Invalid UTF-8 sequence"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = "dzaaaaaa";
        let plaintext_str = "https://github.com/Tualin14/Tualin14";
        let cipher_text = _encrypt(key, plaintext_str).unwrap();
        println!("{:}", cipher_text);
        let decrypted_text = decrypt(key, cipher_text.as_str()).unwrap();
        println!("{:}", decrypted_text);
        assert_eq!(decrypted_text, plaintext_str);
    }

    #[test]
    fn test_decrypt_valid_input() {
        let key = "dzaaaaaa";
        let cipher_text = "SpyDrUrKHAmJ1NTXMMdJ4PkNShp0cKEH-b-_MiMUx_0Osr6XQoPiY6RRqIv08mnfwc9HEmDYzuhEdc_fFiWzKQ";

        match decrypt(key, cipher_text) {
            Ok(decrypted_text) => {
                assert_eq!(decrypted_text, "https://github.com/Tualin14/Tualin14");
            }
            Err(_) => {
                panic!("Decryption failed unexpectedly");
            }
        }
    }

    #[test]
    fn test_decrypt_invalid_input() {
        let key = "dzaaaaaa";
        let invalid_cipher_text = "233SpyDrUrKHAmJ1NTXMMdJ4PkNShp0cKEH-b-_MiMUx_0Osr6XQoPiY6RRqIv08mnfwc9HEmDYzuhEdc_fFiWzKQ";

        match decrypt(key, invalid_cipher_text) {
            Ok(_) => {
                panic!("Decryption unexpectedly succeeded with invalid input");
            }
            Err(error_msg) => {
                assert_eq!(error_msg, "Decoding error");
            }
        }
    }
}
