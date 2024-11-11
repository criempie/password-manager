use rand::Rng;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 !\"#$%&'()*+,-./:;<=>?@[\\]^_`|}~";

pub fn generate(length: u8) -> String {
    let mut rng = rand::thread_rng();
    let mut password = Vec::with_capacity(length as usize);
    let alphabet_length = ALPHABET.chars().count();

    password.resize_with(password.capacity(), || char::from(0));
    for char in password.iter_mut() {
        let index = rng.gen_range(0..alphabet_length);
        *char = ALPHABET.chars().nth(index).unwrap();
    }

    return password.iter().collect();
}
