use serde::{de::DeserializeOwned, Serialize};

pub trait IDatabaseFormat: Serialize + DeserializeOwned {
  fn new() -> Self;
}
