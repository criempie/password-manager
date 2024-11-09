use rand::Rng;

const PASSWORD_LENGTH: u8 = 32;
const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 !\"#$%&'()*+,-./:;<=>?@[\\]^_`|}~";

fn main() {
    let mut rng = rand::thread_rng();

    let mut password_raw = [0; PASSWORD_LENGTH as usize];

    let alphabet_len = ALPHABET.chars().count();
    for value in password_raw.iter_mut() {
        *value = rng.gen_range(0..alphabet_len);
    }

    let mut password = ['0'; PASSWORD_LENGTH as usize];
    for (i, value) in password_raw.iter_mut().enumerate() {
        password[i] = ALPHABET.chars().nth(*value).unwrap();
    }

    println!("{:?}", password.iter().collect::<String>());
}
