pub mod aes;
pub mod error;

pub trait IEncryptionManager {
  type Params;

  fn encrypt(&self, data: &[u8], params: &Self::Params) -> Result<Vec<u8>, error::Error>;
  fn decrypt(&self, data: &[u8], params: &Self::Params) -> Result<Vec<u8>, error::Error>;
}
