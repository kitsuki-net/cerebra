use rusqlite::{params, Connection, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn create_db_tables(db_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let statements = [
        "CREATE TABLE IF NOT EXISTS Note (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL,
            source_id INTEGER NOT NULL,
            context_id INTEGER NOT NULL,
            topic_id INTEGER NOT NULL,
            project_id INTEGER,
            FOREIGN KEY (source_id) REFERENCES Source(id),
            FOREIGN KEY (context_id) REFERENCES Context(id),
            FOREIGN KEY (topic_id) REFERENCES Topic(id)
        )",
        "CREATE TABLE IF NOT EXISTS Topic (
            id INTEGER PRIMARY KEY,
            name CHAR(50) NOT NULL,
            parent_topic_id INTEGER,
            FOREIGN KEY (parent_topic_id) REFERENCES Topic(id)
        )",
        "CREATE TABLE IF NOT EXISTS Context (
            id INTEGER PRIMARY KEY,
            name CHAR(50) NOT NULL,
            parent_context_id INTEGER,
            FOREIGN KEY (parent_context_id) REFERENCES Context(id)
        )",
        "CREATE TABLE IF NOT EXISTS Source (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        )",
        "CREATE TABLE IF NOT EXISTS Project (
            id INTEGER PRIMARY KEY,
            name CHAR(50) NOT NULL,
            topic_id INTEGER,
            idea_id INTEGER,
            FOREIGN KEY (topic_id) REFERENCES Topic(id),
            FOREIGN KEY (idea_id) REFERENCES Idea(id)
        )",
        "CREATE TABLE IF NOT EXISTS Task (
            id INTEGER PRIMARY KEY,
            name CHAR(50) NOT NULL,
            project_id INTEGER,
            FOREIGN KEY (project_id) REFERENCES Project(id)
        )",
        "CREATE TABLE IF NOT EXISTS Idea (
            id INTEGER PRIMARY KEY,
            name CHAR(50) NOT NULL,
            topic_id INTEGER,
            project_id INTEGER,
            FOREIGN KEY (topic_id) REFERENCES Topic(id),
            FOREIGN KEY (project_id) REFERENCES Project(id)
        )",
        "CREATE TABLE IF NOT EXISTS Writing (
            id INTEGER PRIMARY KEY,
            name CHAR(50) NOT NULL,
            project_id INTEGER,
            note_id INTEGER,
            FOREIGN KEY (project_id) REFERENCES Project(id),
            FOREIGN KEY (note_id) REFERENCES Note(id)
        )",
        "CREATE TABLE IF NOT EXISTS Code (
            id INTEGER PRIMARY KEY,
            name CHAR(50) NOT NULL,
            language_id INTEGER NOT NULL,
            project_id INTEGER,
            note_id INTEGER,
            FOREIGN KEY (project_id) REFERENCES Project(id),
            FOREIGN KEY (note_id) REFERENCES Note(id),
            FOREIGN KEY (language_id) REFERENCES CodeLanguage(id)
        )",
        "CREATE TABLE IF NOT EXISTS CodeLanguage (
            id INTEGER PRIMARY KEY,
            name CHAR(50) NOT NULL
        )",
        "CREATE TABLE IF NOT EXISTS NoteHasTopic (
            note_id INTEGER,
            topic_id INTEGER,
            FOREIGN KEY (note_id) REFERENCES Note(id),
            FOREIGN KEY (topic_id) REFERENCES Topic(id)
        )",
        "CREATE TABLE IF NOT EXISTS NoteHasContext (
            note_id INTEGER,
            context_id INTEGER,
            FOREIGN KEY (note_id) REFERENCES Note(id),
            FOREIGN KEY (context_id) REFERENCES Context(id)
        )",
        "CREATE TABLE IF NOT EXISTS NoteHasSource (
            note_id INTEGER,
            source_id INTEGER,
            FOREIGN KEY (note_id) REFERENCES Note(id),
            FOREIGN KEY (source_id) REFERENCES Source(id)
        )",
        "CREATE TABLE IF NOT EXISTS TopicHasTopic (
            topic_id INTEGER,
            parent_topic_id INTEGER,
            FOREIGN KEY (topic_id) REFERENCES Topic(id),
            FOREIGN KEY (parent_topic_id) REFERENCES Topic(id)
        )",
        "CREATE TABLE IF NOT EXISTS ContextHasContext (
            context_id INTEGER,
            parent_context_id INTEGER,
            FOREIGN KEY (context_id) REFERENCES Context(id),
            FOREIGN KEY (parent_context_id) REFERENCES Context(id)
        )",
        "CREATE TABLE IF NOT EXISTS ProjectHasTopic (
            project_id INTEGER,
            topic_id INTEGER,
            FOREIGN KEY (project_id) REFERENCES Project(id),
            FOREIGN KEY (topic_id) REFERENCES Topic(id)
        )",
        "CREATE TABLE IF NOT EXISTS ProjectHasIdea (
            project_id INTEGER,
            idea_id INTEGER,
            FOREIGN KEY (project_id) REFERENCES Project(id),
            FOREIGN KEY (idea_id) REFERENCES Idea(id)
        )",
        "CREATE TABLE IF NOT EXISTS TaskHasProject (
            task_id INTEGER,
            project_id INTEGER,
            FOREIGN KEY (task_id) REFERENCES Task(id),
            FOREIGN KEY (project_id) REFERENCES Project(id)
        )",
        "CREATE TABLE IF NOT EXISTS IdeaHasTopic (
            idea_id INTEGER,
            topic_id INTEGER,
            FOREIGN KEY (idea_id) REFERENCES Idea(id),
            FOREIGN KEY (topic_id) REFERENCES Topic(id)
        )",
        "CREATE TABLE IF NOT EXISTS IdeaHasTask (
            idea_id INTEGER,
            project_id INTEGER,
            FOREIGN KEY (idea_id) REFERENCES Idea(id),
            FOREIGN KEY (project_id) REFERENCES Project(id)
        )",
        "CREATE TABLE IF NOT EXISTS WritingHasProject (
            writing_id INTEGER,
            project_id INTEGER,
            FOREIGN KEY (writing_id) REFERENCES Writing(id),
            FOREIGN KEY (project_id) REFERENCES Project(id)
        )",
        "CREATE TABLE IF NOT EXISTS WritingHasNote (
            writing_id INTEGER,
            note_id INTEGER,
            FOREIGN KEY (writing_id) REFERENCES Writing(id),
            FOREIGN KEY (note_id) REFERENCES Note(id)
        )",
        "CREATE TABLE IF NOT EXISTS CodeHasProject (
            code_id INTEGER,
            project_id INTEGER,
            FOREIGN KEY (code_id) REFERENCES Code(id),
            FOREIGN KEY (project_id) REFERENCES Project(id)
        )",
        "CREATE TABLE IF NOT EXISTS CodeHasNote (
            code_id INTEGER,
            note_id INTEGER,
            FOREIGN KEY (code_id) REFERENCES Code(id),
            FOREIGN KEY (note_id) REFERENCES Note(id)
        )",
        "CREATE TABLE IF NOT EXISTS CodeHasLanguage (
            code_id INTEGER,
            language_id INTEGER,
            FOREIGN KEY (code_id) REFERENCES Code(id),
            FOREIGN KEY (language_id) REFERENCES CodeLanguage(id)
        )",
    ];

    let conn = Connection::open(db_path)?;
    for statement in &statements {
        if let Err(e) = conn.execute(statement, []) {
            eprintln!("Error executing statement: {}\nError: {}", statement, e);
            fs::remove_file(db_path)?;
            return Err(Box::new(e));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use std::env;

    #[test]
    fn test_if_db_tables() {
        let args: Vec<String> = env::args().collect();
        println!("{:?}", args);
        let conn = Connection::open(args[args.len() - 1].clone())
            .expect("Failed to open in-memory database");
        let result = create_db_tables(&conn);
        assert!(result.is_ok());

        // Verify that tables were created
        let tables = [
            "Note",
            "Topic",
            "Context",
            "Source",
            "Project",
            "Task",
            "Idea",
            "Writing",
            "Code",
            "CodeLanguage",
            "NoteHasTopic",
            "NoteHasContext",
            "NoteHasSource",
            "TopicHasTopic",
            "ContextHasContext",
            "ProjectHasTopic",
            "ProjectHasIdea",
            "TaskHasProject",
            "IdeaHasTopic",
            "IdeaHasTask",
            "WritingHasProject",
            "WritingHasNote",
            "CodeHasProject",
            "CodeHasNote",
            "CodeHasLanguage",
        ];

        for table in &tables {
            let query = format!(
                "SELECT name FROM sqlite_master WHERE type='table' AND name='{}';",
                table
            );
            let mut stmt = conn.prepare(&query).expect("Failed to prepare statement");
            let table_exists: Option<String> = stmt.query_row([], |row| row.get(0)).ok();
            assert_eq!(
                table_exists,
                Some(table.to_string()),
                "Table {} does not exist",
                table
            );
        }
    }
}

//pub fn get_id_or_create(
//    conn: &Connection,
//    table: &str,
//    name: &str,
//) -> Result<i64, Box<dyn std::error::Error>> {
//    let query = format!("SELECT id FROM {} WHERE name = ?;", table);
//    let mut stmt = conn.prepare(&query)?;
//    let id: Option<i64> = stmt.query_row(&[&name], |row| row.get(0)).unwrap_or(None);
//
//    match id {
//        Some(id) => Ok(id),
//        None => {
//            let query = format!("INSERT INTO {} (name) VALUES (?);", table);
//            conn.execute(&query, &[&name])?;
//            let id: i64 = conn.last_insert_rowid();
//            Ok(id)
//        }
//    }
//}

//pub fn create_insert_sql(
//    table: &str,
//    required: HashMap<&str, &str>,
//    optional: HashMap<&str, &str>,
//) -> String {
//    let mut sql = String::from(format!("INSERT INTO {} (", table));
//    let mut values = String::from("VALUES (?1");
//
//    for (key, value) in required.iter() {
//        if value.is_empty() {
//            panic!("{} is required", key);
//        }
//        sql.push_str(&format!(", {}", key));
//        values.push_str(&format!(", ?{}", value));
//    }
//
//    for (key, value) in optional.iter() {
//        if !value.is_empty() {
//            sql.push_str(&format!(", {}", key));
//            values.push_str(&format!(", ?{}", value));
//        }
//    }
//
//    sql.push_str(") ");
//    values.push_str(")");
//
//    sql.push_str(&values);
//
//    sql
//}

pub fn get_parents(tag: &str) -> Result<(Vec<&str>, &str), Box<dyn std::error::Error>> {
    let mut parts: Vec<&str> = tag.split('/').collect::<Vec<&str>>();

    let child = parts.pop();

    Ok((parts, child.unwrap()))
}

pub fn get_content_by_id(
    conn: &Connection,
    table: &str,
    row: &str,
    id: i32,
) -> Result<String, rusqlite::Error> {
    let query = format!("SELECT {} FROM {} WHERE id = ?", row, table);
    let mut stmt = conn.prepare(&query)?;
    let content: String = stmt
        .query_row(params![id], |row| row.get(0))
        .unwrap_or_default();
    Ok(content)
}

pub fn get_tags(elements: &[String]) -> HashMap<String, String> {
    let mut hashmap = HashMap::new();

    if elements.iter().any(|element| !element.contains(':')) {
        eprintln!("Invalid tag format. Use key:value pairs.");
        std::process::exit(1);
    }

    for element in elements {
        let parts: Vec<&str> = element.split(':').collect();
        if parts.len() == 2 {
            hashmap.insert(parts[0].to_string(), parts[1].to_string());
        }
    }

    hashmap
}
