use vault::Vault;

pub mod error;
mod vault;

fn main() {
    let mut vault = Vault::new();
    vault.init();

    println!("{:?}", vault.entries);
    println!("{:?}", vault.settings);

    vault.create_entry("hello world".to_string(), "qwerty12345".to_string());
    let _ = vault.save();
}
