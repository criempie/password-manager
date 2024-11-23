use std::io::{self, stdin, stdout, Write};

use crate::vault::{Vault, VaultEntry};

enum Command {
    ShowAllEntries,
    AddNewEntry,
}

pub struct CLI {
    vault: Vault,
    master_key: Option<String>,
}

impl CLI {
    pub fn new(vault: Vault) -> CLI {
        return CLI {
            vault,
            master_key: None,
        };
    }

    pub fn start(&mut self) {
        self.vault.load_from_db().unwrap();

        // if let Ok(master_key) = self.prompt_master_key() {
        //     self.master_key = Some(master_key);
        // }

        match self.prompt_action() {
            Ok(command) => match command {
                Command::ShowAllEntries => {
                    println!("{:?}", self.vault.entries_get());
                }
                Command::AddNewEntry => {
                    let entry = self.prompt_new_entry().unwrap();

                    println!("Created new entry: {:?}", &entry);

                    self.vault.entry_save(entry);
                    self.vault.save_to_db().unwrap();
                }
            },
            Err(err) => panic!("{}", err),
        }
    }

    fn prompt_master_key(&mut self) -> Result<String, io::Error> {
        let mut buffer = String::new();

        print!("Enter master key: ");
        stdout().flush()?;
        stdin().read_line(&mut buffer)?;

        return Ok(buffer);
    }

    fn prompt_action(&mut self) -> Result<Command, io::Error> {
        let mut buffer = String::new();

        println!("Select action:");
        println!("1. Show all entries");
        println!("2. Create new entry");
        stdin().read_line(&mut buffer)?;

        let input = buffer.trim();

        if input == "1" {
            return Ok(Command::ShowAllEntries);
        } else if input == "2" {
            return Ok(Command::AddNewEntry);
        }

        return self.prompt_action();
    }

    fn prompt_new_entry(&mut self) -> Result<VaultEntry, io::Error> {
        let mut login_buf = String::new();
        let mut password_buf = String::new();

        println!("Enter login: ");
        stdin().read_line(&mut login_buf)?;

        println!("Enter password: ");
        stdin().read_line(&mut password_buf)?;

        return Ok(self
            .vault
            .entry_create(login_buf.trim(), password_buf.trim()));
    }
}
