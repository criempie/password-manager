use ui::cli::CLIManager;
use vault::Vault;

pub mod error;
mod ui;
mod vault;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut vault = Vault::new();
  vault.init();

  let cli = CLIManager::new(vault);

  cli.start();

  // vault.create_entry("hello world".to_string(), "123123123".to_string())?;
  // let _ = vault.delete_entry(&"732f005db2944f53bd46efff25b3e9c1".to_string());
  // let _ = vault.save();
  // let entry = vault
  //     .get_entry(&"3b27be489a6c4eeead032463c4b25038".to_string())
  //     .unwrap();
  // println!("{:?}", entry);

  return Ok(());
}
