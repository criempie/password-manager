use aes::{
  cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit},
  Aes256,
};
use rand::Rng;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::vault::base64string::Base64String;

use super::{error, IEncryptionManager};

type KeyBytes = [u8; 32];
type IVBytes = [u8; 16];

pub struct AESEncryptionManager {
  key: EncryptionKey,
}

pub struct AESEncryptionParams {
  iv: EncryptionIV,
}

impl AESEncryptionManager {
  pub fn new(key: EncryptionKey) -> Self {
    Self { key }
  }
}

impl IEncryptionManager for AESEncryptionManager {
  type Params = AESEncryptionParams;

  fn encrypt(&self, data: &[u8], params: &Self::Params) -> Result<Vec<u8>, error::Error> {
    return encrypt_aes_cbc(data, &self.key.bytes(), &params.iv.bytes());
  }

  fn decrypt(&self, data: &[u8], params: &Self::Params) -> Result<Vec<u8>, error::Error> {
    return decrypt_aes_cbc(data, &self.key.bytes(), &params.iv.bytes());
  }
}

fn encrypt_aes_cbc(data: &[u8], key: &KeyBytes, iv: &IVBytes) -> Result<Vec<u8>, error::Error> {
  type Encryptor = cbc::Encryptor<Aes256>;

  // Create a buffer with enough space for padding
  let mut buffer = vec![0u8; data.len() + 16];
  buffer[..data.len()].copy_from_slice(data);

  let cipher = cbc::Encryptor::<Aes256>::new_from_slices(key, iv)
    .map_err(|_| error::Error::ParamsInvalidLength)?;

  let encrypted_data = cipher
    .encrypt_padded_mut::<Pkcs7>(&mut buffer, data.len())
    .map_err(|_| error::Error::PaddingOutputBufferInvalid)?;

  return Ok(encrypted_data.to_vec());
}

fn decrypt_aes_cbc(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, error::Error> {
  type Decryptor = cbc::Decryptor<Aes256>;

  let cipher =
    Decryptor::new_from_slices(key, iv).map_err(|_| error::Error::ParamsInvalidLength)?;

  let mut buffer = data.to_vec();

  cipher
    .decrypt_padded_mut::<Pkcs7>(&mut buffer)
    .map_err(|_| error::Error::UnPaddingError)?;

  return Ok(buffer);
}

#[derive(Clone, Debug)]
pub struct EncryptionKey(KeyBytes);
#[derive(Clone, Debug)]
pub struct EncryptionIV(IVBytes);

impl EncryptionKey {
  pub fn random() -> Self {
    let mut rng = rand::thread_rng();
    let mut key = [0u8; 32];

    rng.fill(&mut key);

    return Self(key);
  }

  pub fn bytes(&self) -> KeyBytes {
    return self.0;
  }
}

impl EncryptionIV {
  pub fn random() -> Self {
    let mut rng = rand::thread_rng();
    let mut iv = [0u8; 16];

    rng.fill(&mut iv);

    return Self(iv);
  }

  pub fn bytes(&self) -> IVBytes {
    return self.0;
  }
}

// impl Serialize for EncryptionKey {
//   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//   where
//     S: Serializer,
//   {
//     return serializer.serialize_str(&Base64String::encode(self.0).0);
//   }
// }

// impl<'de> Deserialize<'de> for EncryptionKey {
//   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//   where
//     D: Deserializer<'de>,
//   {
//     let base64_data = String::deserialize(deserializer)?;
//     let data = Base64String::from(base64_data).decode().unwrap();

//     return Ok(EncryptionKey(data.try_into().unwrap()));
//   }
// }

// impl Serialize for EncryptionIV {
//   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//   where
//     S: Serializer,
//   {
//     return serializer.serialize_str(&Base64String::encode(self.0.to_vec()).0);
//   }
// }

// impl<'de> Deserialize<'de> for EncryptionIV {
//   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//   where
//     D: Deserializer<'de>,
//   {
//     let base64_data = String::deserialize(deserializer)?;
//     let data = Base64String::from(base64_data).decode().unwrap();

//     return Ok(EncryptionIV(data.try_into().unwrap()));
//   }
// }
