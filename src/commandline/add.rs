use crate::config;
use crate::database;
use rusqlite::Connection;
use std::error::Error;

pub fn add(
    config: &config::Config,
    entry_type: &str,
    content: &str,
    tags: Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(&config.db_path).expect("Failed to open database");
    let tags = database::init::get_tags(&tags);

    match entry_type {
        "note" => database::note::add(&config.note_path, &conn, content, tags),
        "idea" => database::idea::add(&conn, content, tags),
        "task" => database::task::add(&conn, content, tags),
        "project" => database::project::add(&conn, content, tags),
        "writings" => database::writing::add(&conn, content, tags),
        "code" => database::code::add(&conn, content, tags),
        _ => {
            eprintln!(
                "Invalid entry type. Use 'note', 'idea', 'task', 'project', 'writings', or 'code'."
            );
            std::process::exit(1);
        }
    }
}
