use rusqlite::{params, Connection, Result};
use std::error::Error;

pub fn get_id(conn: &Connection, name: &str) -> Result<i64, Box<dyn Error>> {
    let query = "SELECT name FROM source WHERE name = ?";
    let mut stmt = conn.prepare(&query).unwrap();
    let id: Option<i64> = stmt
        .query_row(params![name], |row| row.get(0))
        .unwrap_or(None);

    match id {
        Some(id) => Ok(id),
        None => {
            let query = "INSERT INTO source (name) VALUES (?)";
            conn.execute(&query, params![name])?;
            let id: i64 = conn.last_insert_rowid();
            Ok(id)
        }
    }
}
