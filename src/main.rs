use vault::Vault;

pub mod error;
mod vault;

fn main() {
    let mut vault = Vault::new();
    vault.init();

    println!("{:?}", vault.entries);
    println!("{:?}", vault.settings);
}
