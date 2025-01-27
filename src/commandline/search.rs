use crate::config;
use std::error::Error;

pub fn search(
    _config: &config::Config,
    _entry_type: &str,
    _query: &str,
    _number: u64,
) -> Result<(), Box<dyn Error>> {
    println!("Coming soon!");
    Ok(())
}
