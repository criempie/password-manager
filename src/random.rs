use rand::Rng;

pub fn generate_random_bytes(count: usize) -> Vec<u8> {
  let mut rng = rand::thread_rng();
  let mut bytes = Vec::with_capacity(count);
  bytes.resize(bytes.capacity(), 0);
  rng.fill(bytes.as_mut_slice());

  return bytes;
}
