use aes::{cipher::block_padding::Pkcs7, Aes256};
use cbc::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use rand::Rng;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::CryptError;

use super::base64string::Base64String;

#[derive(Copy, Clone, Debug)]
pub struct EncryptionIV([u8; 16]);

impl EncryptionIV {
    pub fn generate_new() -> Self {
        Self(EncryptionIV::generate_random_iv())
    }

    pub fn from(iv: [u8; 16]) -> Self {
        Self(iv)
    }

    fn generate_random_iv() -> [u8; 16] {
        let mut rng = rand::thread_rng();
        let mut iv = [0u8; 16];

        rng.fill(&mut iv);

        return iv;
    }
}

impl Serialize for EncryptionIV {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        return serializer.serialize_str(&Base64String::encode(self.0.to_vec()).0);
    }
}

impl<'de> Deserialize<'de> for EncryptionIV {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let base64_data = String::deserialize(deserializer)?;
        let data = Base64String::from(base64_data).decode().unwrap();

        return Ok(EncryptionIV(data.try_into().unwrap()));
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EncryptionKey([u8; 32]);

impl EncryptionKey {
    pub fn generate_new() -> Self {
        Self(EncryptionKey::generate_random_key())
    }

    pub fn from(key: [u8; 32]) -> Self {
        Self(key)
    }

    pub fn encrypt(&self, iv: &EncryptionIV, data: &[u8]) -> Result<Vec<u8>, CryptError> {
        return encrypt_aes_cbc(&data, &self.0, &iv.0);
    }

    pub fn decrypt(&self, iv: &EncryptionIV, data: Vec<u8>) -> Vec<u8> {
        return decrypt_aes_cbc(data, &self.0, &iv.0);
    }

    fn generate_random_key() -> [u8; 32] {
        let mut rng = rand::thread_rng();
        let mut key = [0u8; 32];

        rng.fill(&mut key);

        return key;
    }
}

impl Serialize for EncryptionKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        return serializer.serialize_str(&Base64String::encode(self.0.to_vec()).0);
    }
}

impl<'de> Deserialize<'de> for EncryptionKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let base64_data = String::deserialize(deserializer)?;
        let data = Base64String::from(base64_data).decode().unwrap();

        return Ok(EncryptionKey(data.try_into().unwrap()));
    }
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
