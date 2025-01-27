use crate::config;
use crate::database;
use rusqlite::Connection;
use std::error::Error;

pub fn mod_entry(
    config: &config::Config,
    entry_type: &str,
    id: u64,
    tags: Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(&config.db_path).expect("Failed to open database");
    let tags = database::init::get_tags(&tags);

    let _ = match entry_type {
        "note" => database::note::modify(&config.note_path, &conn, id, tags),
        "idea" => database::idea::modify(&conn, tags),
        "task" => database::task::modify(&conn, tags),
        "project" => database::project::modify(&conn, tags),
        "writings" => database::writing::modify(&conn, tags),
        "code" => database::code::modify(&conn, tags),
        _ => {
            eprintln!(
                "Invalid entry type. Use 'note', 'idea', 'task', 'project', 'writings', or 'code'."
            );
            std::process::exit(1);
        }
    };

    Ok(())
}
