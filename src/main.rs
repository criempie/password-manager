use std::{
    fs::{self},
    io::{self},
    time::{self, Instant, SystemTime},
};

mod password_generator;
mod ui;
mod vault;

use ui::cli::CLI;
use vault::Vault;

fn main() {
    let mut ui = CLI::new(Vault::new());

    ui.start();
}
