use rusqlite::ffi::SQLITE_NULL;
use rusqlite::{params, Connection, Result};
use std::error::Error;

pub fn get_id(
    conn: &Connection,
    name: &str,
    mut parents: Vec<&str>,
) -> Result<i64, Box<dyn Error>> {
    let query = "SELECT id FROM topic WHERE name = ?";
    let mut stmt = conn.prepare(query)?;
    let id: Option<i64> = stmt
        .query_row(params![name], |row| row.get(0))
        .unwrap_or(None);

    let next_parent = parents.pop();
    let mut parent_topic_id = SQLITE_NULL as i64;
    if let Some(parent) = next_parent {
        parent_topic_id = get_id(conn, parent, parents)?;
    }

    match id {
        Some(id) => Ok(id),
        None => {
            let query = "INSERT INTO topic (name, parent_topic_id) VALUES (?, ?)";
            conn.execute(query, params![name, parent_topic_id])?;
            let id: i64 = conn.last_insert_rowid();
            Ok(id)
        }
    }
}
