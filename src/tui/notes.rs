use crate::{BASE, GREEN, LAVENDER, PEACH, TEAL};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use rusqlite::{types::Value, Connection};
use std::error::Error;
use std::path::Path;

pub fn start(db_path: &Path) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(db_path)?;
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table'")?;
    let table_names: Vec<String> = stmt
        .query_map([], |row| row.get(0))?
        .filter_map(Result::ok)
        .collect();

    let mut entries = Vec::new();
    for table_name in table_names {
        let table_entries = get_table_entries(&conn, &table_name)?;
        entries.push(format!("{}:\n{}", table_name, table_entries.join("\n")));
    }

    display_notes(entries)?;

    Ok(())
}

fn get_table_entries(conn: &Connection, table_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut stmt = conn.prepare(&format!("SELECT * FROM {}", table_name))?;
    let column_names: Vec<String> = stmt.column_names().iter().map(|s| s.to_string()).collect();
    let rows = stmt.query_map([], |row| {
        let mut entry = String::new();
        for (i, column_name) in column_names.iter().enumerate() {
            if i > 0 {
                entry.push_str(", ");
            }
            let value: Value = row.get(i)?;
            entry.push_str(&format!("{}: {}", column_name, value_to_string(value)));
        }
        Ok(format!("- {}", entry))
    })?;

    let mut entries = Vec::new();
    for entry in rows {
        entries.push(entry?);
    }
    Ok(entries)
}

fn value_to_string(value: Value) -> String {
    match value {
        Value::Null => "NULL".to_string(),
        Value::Integer(i) => i.to_string(),
        Value::Real(r) => r.to_string(),
        Value::Text(t) => t,
        Value::Blob(_) => "[BLOB]".to_string(),
    }
}

fn display_notes(entries: Vec<String>) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let size = f.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(92), Constraint::Percentage(8)].as_ref())
                .split(size);

            let items: Vec<ListItem> = entries
                .iter()
                .map(|entry| {
                    let lines: Vec<Line> = entry
                        .split('\n')
                        .map(|line| {
                            if line.contains(":") {
                                let parts: Vec<&str> = line.split(':').collect();
                                let key = parts[0].trim();
                                let value = parts[1..].join(":").trim().to_string(); // Convert to String
                                Line::from(vec![
                                    Span::styled(
                                        key,
                                        Style::default().fg(TEAL).add_modifier(Modifier::BOLD),
                                    ),
                                    Span::raw(": "),
                                    Span::styled(value, Style::default().fg(GREEN)),
                                ])
                            } else {
                                Line::from(Span::styled(
                                    line,
                                    Style::default().fg(PEACH).add_modifier(Modifier::BOLD),
                                ))
                            }
                        })
                        .collect();
                    ListItem::new(lines)
                })
                .collect();

            let list = List::new(items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Note Structure")
                    .style(Style::default().bg(BASE)),
            );
            f.render_widget(list, chunks[0]);

            let help_message = Paragraph::new("q: Quit | ↑/↓: Navigate | Enter: Select")
                .style(Style::default().fg(LAVENDER))
                .block(Block::default().borders(Borders::ALL).title("Help"));

            f.render_widget(help_message, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
