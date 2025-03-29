mod vault;
use vault::{PasswordEntry, Vault_reborn};

fn main() {
    // let mut ui = CLI::new(Vault::new());

    // ui.start();

    let mut vault = Vault_reborn::new();
    let entry = PasswordEntry::new(
        Vault_reborn::generate_id(),
        String::from("Hello world"),
        String::from("12345sdfkj234"),
    );

    let id = entry.id.clone();
    vault.add_entry(entry);

    println!("{:?}", vault.get_entry(id));
}
