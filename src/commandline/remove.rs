use crate::config::Config;
use crate::database::{code, idea, note, project, task, writing};
use rusqlite::Connection;
use std::error::Error;

pub fn rm(config: &Config, entry_type: &str, id: u64) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(&config.db_path)?;

    let _ = match entry_type {
        "note" => note::remove(&config.note_path, &conn, id),
        "idea" => idea::remove(&conn, id),
        "task" => task::remove(&conn, id),
        "project" => project::remove(&conn, id),
        "writings" => writing::remove(&conn, id),
        "code" => code::remove(&conn, id),
        _ => {
            eprintln!(
                "Invalid entry type. Use 'note', 'idea', 'task', 'project', 'writings', or 'code'."
            );
            std::process::exit(1);
        }
    };

    Ok(())
}
