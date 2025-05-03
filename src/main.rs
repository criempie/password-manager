use ui::cli::manager::Manager;

pub mod random;
mod ui;
mod vault;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let _manager = Manager::start();

  return Ok(());
}
