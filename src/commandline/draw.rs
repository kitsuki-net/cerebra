use crate::{BASE, FLAMINGO, LAVENDER, MANTLE, MAROON, PEACH, RED, ROSEWATER, TEXT, YELLOW};
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseEventKind,
};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    prelude::*,
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use rusqlite::Connection;
use std::error::Error as StdError;
use std::io;

#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    name: String,
    columns: Vec<Column>,
    foreign_keys: Vec<ForeignKey>,
    indices: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct Column {
    name: String,
    data_type: String,
}

#[derive(Debug, Clone, PartialEq)]
struct ForeignKey {
    table: String,
    from: String,
    to: String,
}

pub fn draw(conn: &Connection) -> Result<(), Box<dyn StdError>> {
    let tables = fetch_tables(conn)?;

    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Main loop
    let mut selected_table: Option<usize> = None;
    loop {
        terminal.draw(|f| {
            draw_ui(f, &tables, selected_table);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Down => {
                    if let Some(selected) = selected_table {
                        if selected < tables.len() - 1 {
                            selected_table = Some(selected + 1);
                        }
                    } else {
                        selected_table = Some(0);
                    }
                }
                KeyCode::Up => {
                    if let Some(selected) = selected_table {
                        if selected > 0 {
                            selected_table = Some(selected - 1);
                        }
                    }
                }
                KeyCode::Enter => {
                    if let Some(selected) = selected_table {
                        if selected == tables.len() - 1 {
                            selected_table = None;
                        } else {
                            selected_table = Some(selected);
                        }
                    }
                }
                _ => {}
            }
        } else if let Event::Mouse(mouse_event) = event::read()? {
            match mouse_event.kind {
                MouseEventKind::Down(_) => {
                    let y = mouse_event.column as usize;
                    if y < tables.len() {
                        selected_table = Some(y);
                    }
                }
                _ => {}
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn fetch_tables(conn: &Connection) -> Result<Vec<Table>, Box<dyn StdError>> {
    let mut tables = Vec::new();
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table'")?;
    let table_names: Vec<String> = stmt
        .query_map([], |row| row.get(0))?
        .filter_map(Result::ok)
        .collect();

    for table in &table_names {
        let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table))?;
        let columns: Vec<Column> = stmt
            .query_map([], |row| {
                Ok(Column {
                    name: row.get(1)?,
                    data_type: row.get(2)?,
                })
            })?
            .filter_map(Result::ok)
            .collect();

        let mut stmt = conn.prepare(&format!("PRAGMA foreign_key_list({})", table))?;
        let foreign_keys: Vec<ForeignKey> = stmt
            .query_map([], |row| {
                Ok(ForeignKey {
                    table: row.get(2)?,
                    from: row.get(3)?,
                    to: row.get(4)?,
                })
            })?
            .filter_map(Result::ok)
            .collect();

        let mut stmt = conn.prepare(&format!("PRAGMA index_list({})", table))?;
        let indices: Vec<String> = stmt
            .query_map([], |row| row.get(1))?
            .filter_map(Result::ok)
            .collect();

        let table = Table {
            name: table.clone(),
            columns,
            foreign_keys,
            indices,
        };

        tables.push(table);
    }

    tables.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(tables)
}

fn draw_ui(f: &mut Frame, tables: &[Table], selected_table: Option<usize>) {
    let size = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(92), Constraint::Percentage(8)].as_ref())
        .split(size);

    let related_table_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(chunks[0]);

    let selected_related_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(related_table_chunks[1]);

    let related_tables: Vec<&Table> = if let Some(selected) = selected_table {
        tables
            .get(selected)
            .map(|table| {
                table
                    .foreign_keys
                    .iter()
                    .filter_map(|fk| tables.iter().find(|t| t.name == fk.table))
                    .collect()
            })
            .unwrap_or_else(Vec::new)
    } else {
        Vec::new()
    };

    let main_tables: Vec<&Table> = tables.iter().collect();

    let render_table_list =
        |f: &mut Frame, area: Rect, tables: &[&Table], selected_table: Option<usize>| {
            let items: Vec<ListItem> = tables
                .iter()
                .enumerate()
                .map(|(_, table)| {
                    let is_selected = selected_table.map_or(false, |selected| {
                        tables.get(selected).map_or(false, |t| t.name == table.name)
                    });

                    let content = vec![Line::from(Span::styled(
                        format!("{}", table.name),
                        Style::default()
                            .fg(if is_selected { MAROON } else { TEXT })
                            .add_modifier(Modifier::BOLD),
                    ))];

                    let mut item = ListItem::new(content);
                    if is_selected {
                        item = item.style(Style::default().bg(BASE));
                    }
                    item
                })
                .collect();

            let list = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Database Schema")
                        .style(Style::default().bg(MANTLE)),
                )
                .highlight_style(Style::default().bg(BASE));

            f.render_widget(list, area);
        };

    let render_selected_table = |f: &mut Frame, area: Rect, table: &Table| {
        let mut content = vec![Line::from(Span::styled(
            format!("{}", table.name),
            Style::default().fg(MAROON).add_modifier(Modifier::BOLD),
        ))];

        content.push(Line::from(Span::styled(
            "  Columns",
            Style::default().fg(FLAMINGO).add_modifier(Modifier::BOLD),
        )));
        for column in &table.columns {
            content.push(Line::from(Span::styled(
                format!("    - {} {}", column.name, column.data_type),
                Style::default().fg(ROSEWATER),
            )));
        }
        content.push(Line::from(Span::styled(
            "  Foreign keys",
            Style::default().fg(PEACH).add_modifier(Modifier::BOLD),
        )));
        for fk in &table.foreign_keys {
            content.push(Line::from(Span::styled(
                format!("    - {}.{} -> {}.{}", fk.table, fk.to, table.name, fk.from),
                Style::default().fg(PEACH),
            )));
        }
        content.push(Line::from(Span::styled(
            format!("  Indices: {:?}", table.indices),
            Style::default().fg(YELLOW),
        )));

        let list = List::new(vec![ListItem::new(content)])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Selected Table")
                    .style(Style::default().bg(MANTLE)),
            )
            .highlight_style(Style::default().bg(BASE));

        f.render_widget(list, area);
    };

    let render_related_table_list = |f: &mut Frame, area: Rect, tables: &[&Table]| {
        let items: Vec<ListItem> = tables
            .iter()
            .map(|table| {
                let mut content = vec![Line::from(Span::styled(
                    format!(
                        "{}{}",
                        table.name,
                        "-".repeat(size.width as usize - table.name.len())
                    ),
                    Style::default().fg(RED).add_modifier(Modifier::BOLD),
                ))];

                for column in &table.columns {
                    content.push(Line::from(Span::styled(
                        format!("  - {} {}", column.name, column.data_type),
                        Style::default().fg(ROSEWATER),
                    )));
                }

                ListItem::new(content)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Related Tables")
                    .style(Style::default().bg(MANTLE)),
            )
            .highlight_style(Style::default().bg(BASE));

        f.render_widget(list, area);
    };

    render_table_list(f, related_table_chunks[0], &main_tables, selected_table);

    if let Some(selected) = selected_table {
        if let Some(table) = tables.get(selected) {
            render_selected_table(f, selected_related_chunks[0], table);
        }
    }

    if !related_tables.is_empty() {
        render_related_table_list(f, selected_related_chunks[1], &related_tables);
    }

    let help_message = Paragraph::new("q: Quit | ↑/↓: Navigate | Enter: Select")
        .style(Style::default().fg(LAVENDER))
        .block(Block::default().borders(Borders::ALL).title("Help"));

    f.render_widget(help_message, chunks[1]);
}
