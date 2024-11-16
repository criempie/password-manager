use std::{
    fs::{self},
    io::{self},
};

mod password_generator;
mod vault;

use vault::database;

fn main() {
    let mut vault = vault::Vault::open();
    let mut database = database::Database::new();

    database.open().unwrap();

    let entry = vault.entry_create(String::from("hello"), String::from("world!!!"));
    let entry2 = vault.entry_create(String::from("hello2"), String::from("123123123!!!"));

    database.load(&vec![entry, entry2]).unwrap();
    let unloaded = database.unload().unwrap();

    println!("{:?}", unloaded);
}
