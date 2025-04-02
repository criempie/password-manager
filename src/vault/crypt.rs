use aes::{cipher::block_padding::Pkcs7, Aes256};
use base64::{engine::general_purpose, Engine as _};
use cbc::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use rand::Rng;

use crate::error::CryptError;

type Aes256CbcEnc = cbc::Encryptor<Aes256>;

// Заменить key, iv : string на слайсы
pub fn first_stage_encrypt(
    data: Vec<u8>,
    key: [u8; 32],
    iv: [u8; 16],
) -> Result<Vec<u8>, CryptError> {
    return encrypt_aes_cbc(data.as_slice(), &key, &iv);
}

pub fn first_stage_decrypt(data: Vec<u8>, key: String, iv: String) -> Vec<u8> {
    return decrypt_aes_cbc(data, key.as_bytes(), iv.as_bytes());
}

pub fn bytes_to_base64(data: Vec<u8>) -> String {
    return general_purpose::STANDARD.encode(data);
}

pub fn base64_to_bytes(encoded: String) -> Result<Vec<u8>, base64::DecodeError> {
    return general_purpose::STANDARD.decode(encoded);
}

pub fn generate_random_key_iv() -> ([u8; 32], [u8; 16]) {
    let mut rng = rand::thread_rng();
    let mut key = [0u8; 32]; // 128 бит для AES128
    let mut iv = [0u8; 16]; // 128 бит для IV

    rng.fill(&mut key);
    rng.fill(&mut iv);

    return (key, iv);
}

type Encryptor = cbc::Encryptor<Aes256>;
type Decryptor = cbc::Decryptor<Aes256>;

fn encrypt_aes_cbc(data: &[u8], key: &[u8; 32], iv: &[u8; 16]) -> Result<Vec<u8>, CryptError> {
    // Create a buffer with enough space for padding
    let mut buffer = vec![0u8; data.len() + 16]; // Worst case: full block of padding
    buffer[..data.len()].copy_from_slice(data);

    let cipher =
        Aes256CbcEnc::new_from_slices(key, iv).map_err(|e| CryptError::Any(e.to_string()))?;

    let encrypted_data = cipher
        .encrypt_padded_mut::<Pkcs7>(&mut buffer, data.len())
        .map_err(|e| CryptError::Any(e.to_string()))?;

    return Ok(encrypted_data.to_vec());
}

fn decrypt_aes_cbc(data: Vec<u8>, key: &[u8], iv: &[u8]) -> Vec<u8> {
    let cipher = Decryptor::new_from_slices(key, iv).unwrap();
    let mut buffer = data;

    cipher.decrypt_padded_mut::<Pkcs7>(&mut buffer).unwrap();

    return buffer;
}
