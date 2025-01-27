use crate::database::init::get_content_by_id;
use rusqlite::{params, Connection};
use std::cmp;
use std::error::Error;
use std::path::Path;

pub fn last(db_path: &Path, amount: u64, entry_type: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(db_path).expect("Failed to open database");

    let query = format!("SELECT * FROM {} ORDER BY id DESC LIMIT ?", entry_type);
    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query(params![amount])?;

    let max_widths = calculate_max_widths(&conn, &mut rows)?;

    // Rewind the iterator to process rows again
    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query(params![amount])?;

    // Print the header
    println!("Last {} {}s:", amount, entry_type);
    println!(
        "| {:<width0$} | {:<width1$} | {:<width2$} | {:<width3$} | {:<width4$} |",
        "id",
        "content",
        "topic",
        "context",
        "source",
        width0 = max_widths[0],
        width1 = max_widths[1],
        width2 = max_widths[2],
        width3 = max_widths[3],
        width4 = max_widths[4]
    );
    println!(
        "|-{:-<width0$}-|-{:-<width1$}-|-{:-<width2$}-|-{:-<width3$}-|-{:-<width4$}-|",
        "",
        "",
        "",
        "",
        "",
        width0 = max_widths[0],
        width1 = max_widths[1],
        width2 = max_widths[2],
        width3 = max_widths[3],
        width4 = max_widths[4]
    );

    // Print the rows
    while let Some(row) = rows.next()? {
        let id = row.get::<usize, i32>(0).unwrap_or_default().to_string();
        let content = row.get::<usize, String>(1).unwrap_or_default();
        let topic_id = row.get::<usize, i32>(4).unwrap_or_default();
        let context_id = row.get::<usize, i32>(3).unwrap_or_default();
        let source_id = row.get::<usize, i32>(2).unwrap_or_default();

        let topic = get_content_by_id(&conn, "topic", "name", topic_id)?;
        let context = get_content_by_id(&conn, "context", "name", context_id)?;
        let source = get_content_by_id(&conn, "source", "name", source_id)?;

        println!(
            "| {:<width0$} | {:<width1$} | {:<width2$} | {:<width3$} | {:<width4$} |",
            id,
            content,
            topic,
            context,
            source,
            width0 = max_widths[0],
            width1 = max_widths[1],
            width2 = max_widths[2],
            width3 = max_widths[3],
            width4 = max_widths[4]
        );
    }

    Ok(())
}

fn calculate_max_widths<'a>(
    conn: &Connection,
    rows: &mut rusqlite::Rows<'a>,
) -> Result<Vec<usize>, rusqlite::Error> {
    let mut max_widths = vec![2, 7, 5, 7, 6]; // Initial widths based on header lengths

    while let Some(row) = rows.next()? {
        let id = row.get::<usize, i32>(0).unwrap_or_default().to_string();
        let content = row.get::<usize, String>(1).unwrap_or_default();
        let topic_id = row.get::<usize, i32>(4).unwrap_or_default();
        let context_id = row.get::<usize, i32>(3).unwrap_or_default();
        let source_id = row.get::<usize, i32>(2).unwrap_or_default();

        let topic = get_content_by_id(conn, "topic", "name", topic_id)?;
        let context = get_content_by_id(conn, "context", "name", context_id)?;
        let source = get_content_by_id(conn, "source", "name", source_id)?;

        max_widths[0] = cmp::max(max_widths[0], id.len());
        max_widths[1] = cmp::max(max_widths[1], content.len());
        max_widths[2] = cmp::max(max_widths[2], topic.len());
        max_widths[3] = cmp::max(max_widths[3], context.len());
        max_widths[4] = cmp::max(max_widths[4], source.len());
    }

    Ok(max_widths)
}
