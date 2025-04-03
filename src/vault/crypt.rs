use aes::{cipher::block_padding::Pkcs7, Aes256};
use base64::{engine::general_purpose, Engine as _};
use cbc::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use rand::Rng;

use crate::error::CryptError;

type Base64String = String;

pub struct EncryptionPair {
    key: [u8; 32],
    iv: [u8; 16],
}

pub struct EncryptionPairFold {
    data: Base64String,
}

impl EncryptionPair {
    // Сгенерировать новую пару (key, iv).
    pub fn generate() -> Self {
        let (key, iv) = Self::generate_random_key_iv();

        return Self { key, iv };
    }

    pub fn new(key: [u8; 32], iv: [u8; 16]) -> Self {
        Self { key, iv }
    }

    // Сжать пару в один массив [u8; 48] с последующим преобразованием в base64.
    pub fn fold(self) -> EncryptionPairFold {
        let mut combination = [0u8; 32 + 16];

        combination[0..32].copy_from_slice(&self.key);
        combination[32..48].copy_from_slice(&self.iv);

        let encoded = bytes_to_base64(&combination);

        return EncryptionPairFold::new(encoded);
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, CryptError> {
        return encrypt_aes_cbc(data, &self.key, &self.iv);
    }

    pub fn decrypt(&self, data: Vec<u8>) -> Vec<u8> {
        return decrypt_aes_cbc(data, &self.key, &self.iv);
    }

    fn generate_random_key_iv() -> ([u8; 32], [u8; 16]) {
        let mut rng = rand::thread_rng();
        let mut key = [0u8; 32];
        let mut iv = [0u8; 16];

        rng.fill(&mut key);
        rng.fill(&mut iv);

        return (key, iv);
    }
}

impl EncryptionPairFold {
    pub fn new(data: String) -> Self {
        Self { data }
    }

    // Декодировать base64 в [u8; 48] и преобразовать в EncryptionPair { key, iv }.
    pub fn unfold(collapsed: Base64String) -> Result<EncryptionPair, CryptError> {
        let decoded =
            base64_to_bytes(collapsed).map_err(|e| CryptError::Base64DecodeError(e.to_string()))?;

        if decoded.len() != 48 {
            return Err(CryptError::FoldInvalidLength);
        }

        let key: [u8; 32] = decoded[0..32].try_into().unwrap();
        let iv: [u8; 16] = decoded[32..48].try_into().unwrap();

        return Ok(EncryptionPair::new(key, iv));
    }
}

pub fn bytes_to_base64(data: &[u8]) -> Base64String {
    return general_purpose::STANDARD.encode(data);
}

pub fn base64_to_bytes(encoded: Base64String) -> Result<Vec<u8>, base64::DecodeError> {
    return general_purpose::STANDARD.decode(encoded);
}

type Encryptor = cbc::Encryptor<Aes256>;
type Decryptor = cbc::Decryptor<Aes256>;

fn encrypt_aes_cbc(data: &[u8], key: &[u8; 32], iv: &[u8; 16]) -> Result<Vec<u8>, CryptError> {
    // Create a buffer with enough space for padding
    let mut buffer = vec![0u8; data.len() + 16]; // Worst case: full block of padding
    buffer[..data.len()].copy_from_slice(data);

    let cipher = cbc::Encryptor::<Aes256>::new_from_slices(key, iv)
        .map_err(|e| CryptError::Any(e.to_string()))?;

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
