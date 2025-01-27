use crate::database;
use fallible_iterator::FallibleIterator;
use rusqlite::{params, Connection};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

pub fn add(
    note_directory: &Path,
    conn: &Connection,
    content: &str,
    tags: HashMap<String, String>,
) -> Result<(), Box<dyn Error>> {
    let mut source = String::new();
    let mut topic = String::new();
    let mut context = String::new();

    for (key, value) in &tags {
        match key.as_str() {
            "source" => source = value.parse().unwrap(),
            "topic" => topic = value.parse().unwrap(),
            "context" => context = value.parse().unwrap(),
            _ => panic!("Invalid key in tags"),
        }
    }

    let last_id = add_to_db(conn, content, &source, &topic, &context)
        .expect("Failed to add note to database");
    println!(
        "Added note {}: '{}' with source '{}', topic '{}', context '{}'",
        last_id, content, source, topic, context
    );

    add_to_notes(note_directory, content, &topic).expect("Failed to add note to notes");

    Ok(())
}

fn add_to_db(
    conn: &Connection,
    content: &str,
    source: &str,
    topic: &str,
    context: &str,
) -> Result<i64, Box<dyn Error>> {
    let source_id = database::source::get_id(&conn, &source).expect("Failed to get source id");

    let (topic_parents, topic_child) =
        database::init::get_parents(&topic).expect("Failed to get topic parents");
    let topic_id = database::topic::get_id(&conn, &topic_child, topic_parents)
        .expect("Failed to get topic id");

    let (context_parents, context_child) =
        database::init::get_parents(&context).expect("Failed to get context parents");
    let context_id = database::context::get_id(&conn, &context_child, context_parents)
        .expect("Failed to get context id");

    let source_id_str: &str = &source_id.to_string();
    let context_id_str: &str = &context_id.to_string();
    let topic_id_str: &str = &topic_id.to_string();
    conn.execute(
        "INSERT INTO note (content, source_id, context_id, topic_id) VALUES (?1, ?2, ?3, ?4)",
        &[&content, source_id_str, context_id_str, &topic_id_str],
    )
    .expect("Failed to insert note");

    Ok(conn.last_insert_rowid())
}

fn add_to_notes(note_directory: &Path, content: &str, topic: &str) -> Result<(), Box<dyn Error>> {
    let (mut topic_parents, topic_child) = database::init::get_parents(&topic)?;

    let mut current_path = note_directory.to_path_buf();
    let file_name = topic_parents.pop().unwrap_or(topic_child);
    for parent in topic_parents {
        current_path.push(parent);
        if !current_path.exists() {
            std::fs::create_dir(&current_path)?;
        }
    }

    current_path.push(format!("{}.md", file_name));
    let file_exists = current_path.exists();
    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&current_path)?;

    let reader = BufReader::new(File::open(&current_path)?);
    let mut lines = Vec::new();
    let mut topic_found = false;

    if !file_exists {
        lines.push(format!("# {}\n", file_name));
    }

    if file_name == topic_child {
        lines.push(format!("## {}\n\n{}", topic_child, content));
        topic_found = true;
    } else {
        for line in reader.lines() {
            let line = line?;
            lines.push(line.clone());
            if line.contains(&format!("## {}", topic_child)) {
                topic_found = true;
                lines.push(content.to_string());
            }
        }
    }

    if !topic_found {
        lines.push(format!("## {}\n\n{}", topic_child, content));
    }

    // Reopen the file in write mode to overwrite the content
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&current_path)?;

    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

pub fn remove(note_directory: &Path, conn: &Connection, id: u64) -> Result<(), Box<dyn Error>> {
    let entry_content: Option<String> = match conn.query_row(
        "SELECT content FROM note WHERE id = ?1",
        params![id.to_string()],
        |row| row.get(0),
    ) {
        Ok(content) => Some(content),
        Err(err) => {
            eprintln!("Failed to get entry content: {}", err);
            std::process::exit(1);
        }
    };

    let mut input = String::new();
    println!(
        "Are you sure you want to delete entry '{}'? (y/n)",
        entry_content.as_deref().unwrap_or("No content")
    );
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if input.trim() != "y" {
        println!("Aborting deletion");
        return Ok(());
    }

    let delete_query = "DELETE FROM note WHERE id = ?";
    conn.execute(delete_query, params![id])?;

    let content = entry_content.unwrap_or("".to_string());
    let mut found = false;
    while !found {
        for entry in std::fs::read_dir(note_directory)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                remove_line_from_file(&path, &content)?;
                found = true;
            }
        }
    }

    Ok(())
}

fn remove_line_from_file<P: AsRef<Path>>(file_path: P, line_content: &str) -> io::Result<()> {
    let file = File::open(&file_path)?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if !line.contains(line_content) {
            lines.push(line);
        }
    }

    let mut file = File::create(&file_path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

pub fn modify(
    note_directory: &Path,
    conn: &Connection,
    id: u64,
    tags: HashMap<String, String>,
) -> Result<(), Box<dyn Error>> {
    let old_content: Option<String> = match conn.query_row(
        "SELECT content FROM note WHERE id = ?1",
        params![id.to_string()],
        |row| row.get(0),
    ) {
        Ok(content) => Some(content),
        Err(err) => {
            eprintln!("Failed to get entry content: {}", err);
            std::process::exit(1);
        }
    };

    modify_db(conn, id, tags.clone()).expect("Failed to modify database entry");

    if tags.get("content").is_some() {
        modify_notes(
            note_directory,
            &old_content.unwrap_or("".to_string()),
            tags.get("content").map_or("", String::as_str),
        )
        .expect("Failed to modify notes entry");
    }

    println!("Modified entry {}", id);
    Ok(())
}

fn modify_db(
    conn: &Connection,
    id: u64,
    tags: HashMap<String, String>,
) -> Result<(), Box<dyn Error>> {
    let mut source = String::new();
    let mut topic = String::new();
    let mut context = String::new();
    let mut content = String::new();

    for (key, value) in &tags {
        match key.as_str() {
            "source" => source = value.parse().unwrap(),
            "topic" => topic = value.parse().unwrap(),
            "context" => context = value.parse().unwrap(),
            "content" => content = value.parse().unwrap(),
            _ => panic!("Invalid key in tags"),
        }
    }

    if content.is_empty() && source.is_empty() && topic.is_empty() && context.is_empty() {
        eprintln!("No tags provided for modification");
        std::process::exit(1);
    }

    // select statement based on id, get the current values
    let select_query = "SELECT content, source_id, topic_id, context_id FROM note WHERE id = ?";
    let mut select_stmt = conn.prepare(select_query)?;
    let select_rows = select_stmt.query(params![id]).unwrap_or_else(|err| {
        eprintln!("Failed to get entry: {}", err);
        std::process::exit(1);
    });

    if let Some((
        mut updated_content,
        mut updated_source_id,
        mut updated_topic_id,
        mut updated_context_id,
    )) = select_rows
        .map(|row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)))
        .next()
        .unwrap_or(None)
    {
        if !source.is_empty() {
            updated_source_id =
                database::source::get_id(&conn, &source).expect("Failed to get source id");
        }
        if !topic.is_empty() {
            let (topic_parents, topic_child) =
                database::init::get_parents(&topic).expect("Failed to get topic parents");
            updated_topic_id = database::topic::get_id(&conn, &topic_child, topic_parents)
                .expect("Failed to get topic id");
        }
        if !context.is_empty() {
            let (context_parents, context_child) =
                database::init::get_parents(&context).expect("Failed to get context parents");
            updated_context_id = database::context::get_id(&conn, &context_child, context_parents)
                .expect("Failed to get context id");
        }
        if !content.is_empty() {
            updated_content = content;
        }

        // prepare and execute update statement
        let update_query =
            "UPDATE note SET content = ?, source_id = ?, topic_id = ?, context_id = ? WHERE id = ?";
        conn.execute(
            update_query,
            params![
                updated_content,
                updated_source_id,
                updated_topic_id,
                updated_context_id,
                id
            ],
        )?;
    } else {
        eprintln!("Failed to get row");
        std::process::exit(1);
    }

    Ok(())
}

fn modify_notes(
    note_directory: &Path,
    old_content: &str,
    new_content: &str,
) -> Result<(), Box<dyn Error>> {
    let mut found = false;
    for entry in std::fs::read_dir(note_directory)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            modify_line_in_file(&path, old_content, new_content)?;
            found = true;
        }
    }

    if !found {
        eprintln!("Failed to find note in notes directory");
        std::process::exit(1);
    }

    Ok(())
}

fn modify_line_in_file<P: AsRef<Path>>(
    file_path: P,
    old_content: &str,
    new_content: &str,
) -> io::Result<()> {
    let file = File::open(&file_path)?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.contains(old_content) {
            lines.push(new_content.to_string());
        } else {
            lines.push(line);
        }
    }

    let mut file = File::create(&file_path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}
