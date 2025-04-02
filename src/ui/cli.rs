use crate::vault::Vault;
use std::io::{stdin, stdout, Write};

enum Command {
    GetEntryById(String),
}

pub struct CLIManager {
    vault: Vault,
}

impl CLIManager {
    pub fn new(vault: Vault) -> Self {
        Self { vault }
    }

    pub fn start(&self) {
        match self.prompt_navigation() {
            Err(e) => {
                println!("{}", e.to_string());
                self.start();
            }
            Ok(command) => match command {
                Command::GetEntryById(id) => {
                    println!("{:?}", self.vault.get_entry(&id));
                }
            },
        }
    }

    fn prompt_navigation(&self) -> Result<Command, Error> {
        stdout().flush().map_err(|e| Error::IO(e.to_string()))?;

        println!("Select action:");
        println!("1. Get entry by id.");

        let mut buffer = String::new();

        stdin()
            .read_line(&mut buffer)
            .map_err(|e| Error::IO(e.to_string()))?;

        let input = buffer.trim();

        match input {
            "1" => {
                let id = self.prompt_entry_id()?;

                return Ok(Command::GetEntryById(id));
            }
            _ => Err(Error::InvalidCommand),
        }
    }

    fn prompt_entry_id(&self) -> Result<String, Error> {
        print!("ID: ");
        stdout().flush().map_err(|e| Error::IO(e.to_string()))?;

        let mut buffer = String::new();

        stdin()
            .read_line(&mut buffer)
            .map_err(|e| Error::IO(e.to_string()))?;

        println!();

        let input = buffer.trim();

        return Ok(input.to_string());
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidCommand,
    IO(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidCommand => write!(f, "CLI error: Invalid command."),
            Self::IO(message) => write!(f, "CLI IO error: {}", message),
        }
    }
}

impl std::error::Error for Error {}
