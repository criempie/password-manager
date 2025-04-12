use aes::{
    cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit},
    Aes256,
};

use super::{error, IEncryptionManager};

type KeyBytes = [u8; 32];
type IVBytes = [u8; 16];

pub struct AESEncryptionManager {
    key: KeyBytes,
}

pub struct AESEncryptionParams {
    iv: IVBytes,
}

impl AESEncryptionManager {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }
}

impl IEncryptionManager for AESEncryptionManager {
    type Params = AESEncryptionParams;

    fn encrypt(&self, data: &[u8], params: &Self::Params) -> Result<Vec<u8>, error::Error> {
        return encrypt_aes_cbc(data, &self.key, &params.iv);
    }

    fn decrypt(&self, data: &[u8], params: &Self::Params) -> Result<Vec<u8>, error::Error> {
        return decrypt_aes_cbc(data, &self.key, &params.iv);
    }
}

pub fn encrypt_aes_cbc(data: &[u8], key: &KeyBytes, iv: &IVBytes) -> Result<Vec<u8>, error::Error> {
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

pub fn decrypt_aes_cbc(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, error::Error> {
    type Decryptor = cbc::Decryptor<Aes256>;

    let cipher =
        Decryptor::new_from_slices(key, iv).map_err(|_| error::Error::ParamsInvalidLength)?;

    let mut buffer = data.to_vec();

    cipher
        .decrypt_padded_mut::<Pkcs7>(&mut buffer)
        .map_err(|_| error::Error::UnPaddingError)?;

    return Ok(buffer);
}
