use rusqlite::Connection;
use std::collections::HashMap;
use std::error::Error;

pub fn add(
    _conn: &Connection,
    _content: &str,
    _tags: HashMap<String, String>,
) -> Result<(), Box<dyn Error>> {
    println!("Coming soon!");
    Ok(())
}

pub fn remove(_conn: &Connection, _id: u64) -> Result<(), Box<dyn Error>> {
    println!("Coming soon!");
    Ok(())
}

pub fn modify(_conn: &Connection, _tags: HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    println!("Coming soon!");
    Ok(())
}
