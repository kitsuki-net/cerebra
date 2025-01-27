use clap::{Parser, Subcommand};
use ratatui::style::Color;
use rusqlite::Connection;
use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};

mod commandline;
mod config;
mod database;
mod tui;

// Define Catppuccin color palette
const ROSEWATER: Color = Color::Rgb(245, 224, 220);
const FLAMINGO: Color = Color::Rgb(242, 205, 205);
//const PINK: Color = Color::Rgb(245, 194, 231);
//const MAUVE: Color = Color::Rgb(203, 166, 247);
const RED: Color = Color::Rgb(243, 139, 168);
const MAROON: Color = Color::Rgb(235, 160, 172);
const PEACH: Color = Color::Rgb(250, 179, 135);
const YELLOW: Color = Color::Rgb(249, 226, 175);
const GREEN: Color = Color::Rgb(166, 227, 161);
const TEAL: Color = Color::Rgb(148, 226, 213);
//const SKY: Color = Color::Rgb(137, 220, 235);
//const SAPPHIRE: Color = Color::Rgb(116, 199, 236);
//const BLUE: Color = Color::Rgb(137, 180, 250);
const LAVENDER: Color = Color::Rgb(180, 190, 254);
const TEXT: Color = Color::Rgb(205, 214, 244);
//const SURFACE: Color = Color::Rgb(41, 41, 62);
const BASE: Color = Color::Rgb(30, 30, 46);
const MANTLE: Color = Color::Rgb(24, 24, 37);

#[derive(Parser)]
#[command(version, about = "a terminal-based knowledge management tool", long_about = None)]
#[command(next_line_help = true)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(
        about = "initialize Cerebra",
        long_about = "create the configuration file, database, and directories"
    )]
    Init {
        // The path to the database file
        #[clap(
            long,
            value_name = "PATH",
            default_value = "$HOME/.cerebra/cerebra.db",
            verbatim_doc_comment
        )]
        db_path: PathBuf,

        // The path to the configuration file
        #[clap(
            long,
            value_name = "PATH",
            default_value = "$HOME/.cerebra/cerebra.conf",
            verbatim_doc_comment
        )]
        config_path: PathBuf,

        // The path to the log file
        #[clap(
            long,
            value_name = "PATH",
            default_value = "$HOME/.cerebra/cerebra.log",
            verbatim_doc_comment
        )]
        log_path: PathBuf,

        // The path to the note directory
        #[clap(
            long,
            value_name = "PATH",
            default_value = "$HOME/cerebra/notes",
            verbatim_doc_comment
        )]
        note_path: PathBuf,

        // The path to the misc directory
        #[clap(
            long,
            value_name = "PATH",
            default_value = "$HOME/cerebra/misc",
            verbatim_doc_comment
        )]
        misc_path: PathBuf,

        // The path to the todo directory
        #[clap(
            long,
            value_name = "PATH",
            default_value = "$HOME/cerebra/todos",
            verbatim_doc_comment
        )]
        todo_path: PathBuf,

        // The path to the journal directory
        #[clap(
            long,
            value_name = "PATH",
            default_value = "$HOME/cerebra/writings",
            verbatim_doc_comment
        )]
        journal_path: PathBuf,

        // The path to the code directory
        #[clap(
            long,
            value_name = "PATH",
            default_value = "$HOME/cerebra/code",
            verbatim_doc_comment
        )]
        code_path: PathBuf,

        // The theme to use for the TUI
        #[clap(long, default_value = "dark", verbatim_doc_comment)]
        theme: String,

        // The editor to use for editing entries
        #[clap(long, default_value = "nvim", verbatim_doc_comment)]
        editor: String,
    },
    #[command(
        about = "display the last entries",
        long_about = None
    )]
    Last {
        // The type of entries to display
        #[clap(index = 1, default_value = "note", verbatim_doc_comment)]
        entry_type: String,

        // The number of entries to display
        #[clap(long, default_value = "30", verbatim_doc_comment)]
        number: u64,
    },
    #[command(about = "add an entry", long_about = None)]
    Add {
        // The type of the entry
        #[clap(index = 1, required = true, value_name = "TYPE", verbatim_doc_comment)]
        entry_type: String,

        // The content of the entry
        #[clap(
            index = 2,
            required = true,
            value_name = "CONTENT",
            verbatim_doc_comment
        )]
        content: String,

        // The tags of the entry, e.g. source:book or topic:it/programming/rust
        #[clap(index = 3, num_args(1..), value_name = "TAGS", verbatim_doc_comment)]
        tags: Vec<String>,
    },
    #[command(
        about = "remove an entry",
        long_about = None
    )]
    Rm {
        // The type of the entry
        #[clap(index = 1, required = true, value_name = "TYPE", verbatim_doc_comment)]
        entry_type: String,

        // The ID of the entry to remove
        #[clap(index = 2, required = true, value_name = "ID", verbatim_doc_comment)]
        id: u64,
    },
    #[command(
        about = "modify an entry",
        long_about = None
    )]
    Mod {
        // The type of the entry
        #[clap(index = 1, required = true, value_name = "TYPE", verbatim_doc_comment)]
        entry_type: String,

        // The ID of the entry to modify
        #[clap(index = 2, required = true, value_name = "ID", verbatim_doc_comment)]
        id: u64,

        // The new tags of the entry
        #[clap(index = 3, num_args(1..), value_name = "TAGS", verbatim_doc_comment)]
        tags: Vec<String>,
    },
    #[command(about = "search for an entry", long_about = None)]
    Search {
        // The type of entries to search for
        #[clap(
            index = 1,
            required = true,
            default_value = "note",
            verbatim_doc_comment
        )]
        entry_type: String,

        // The query to search for
        #[clap(index = 2, required = true, value_name = "QUERY", verbatim_doc_comment)]
        query: String,

        // The number of entries to display
        #[clap(long, default_value = "30", verbatim_doc_comment)]
        number: u64,
    },
    #[command(
        about = "draw a graph",
        long_about = "draw a graph of either the database or relationships"
    )]
    Draw,
    #[command(
        about = "get information about Cerebra",
        long_about = "get a long welcome message, tutorial (coming soon) and some tips"
    )]
    Intro,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let config_path = config::get_config_path();
    let config: config::Config = match &config_path {
        Some(path) => config::read_config(&path).expect("Failed to read config file"),
        None => config::Config::default(),
    };

    match &args.command {
        Some(Commands::Init {
            db_path,
            log_path,
            config_path,
            note_path,
            misc_path,
            todo_path,
            journal_path,
            code_path,
            theme,
            editor,
        }) => {
            let config: config::Config = config::Config::new(
                resolve_path(db_path)?,
                resolve_path(log_path)?,
                resolve_path(note_path)?,
                resolve_path(misc_path)?,
                resolve_path(todo_path)?,
                resolve_path(journal_path)?,
                resolve_path(code_path)?,
                theme.clone(),
                editor.clone(),
            );

            let config_path: &Path = &resolve_path(config_path)?;

            commandline::init::init(config_path, &config).expect("Failed to initialize Cerebra")
        }
        Some(Commands::Last { number, entry_type }) => {
            check_cerebra(&config);
            commandline::last::last(&config.db_path, *number, entry_type)
                .expect("Failed to get last entries")
        }
        Some(Commands::Add {
            entry_type,
            content,
            tags,
        }) => {
            check_cerebra(&config);
            commandline::add::add(&config, entry_type, content, tags.clone())?
        }
        Some(Commands::Rm { entry_type, id }) => {
            check_cerebra(&config);
            commandline::remove::rm(&config, entry_type, *id).expect("Failed to remove entry")
        }
        Some(Commands::Mod {
            entry_type,
            id,
            tags,
        }) => commandline::modify::mod_entry(&config, entry_type, *id, tags.clone())
            .expect("Failed to modify entry"),
        Some(Commands::Search {
            entry_type,
            query,
            number,
        }) => {
            check_cerebra(&config);
            commandline::search::search(&config, entry_type, query, *number)
                .expect("Failed to search for entry")
        }
        Some(Commands::Draw) => {
            let conn = Connection::open(&config.db_path).expect("Failed to open database");
            commandline::draw::draw(&conn).expect("Failed to draw graph");
        }
        Some(Commands::Intro) => commandline::intro::intro()?,
        None => {
            check_cerebra(&config);
            tui::notes::start(&config.db_path)?;
        }
    }

    Ok(())
}

fn resolve_path(path: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let home = env::var("HOME")?;
    let home_path = PathBuf::from(home);
    let current_dir = env::current_dir()?;
    let mut path: PathBuf = PathBuf::from(path);

    if path.starts_with("~") {
        path = home_path.join(path.strip_prefix("~").unwrap());
    } else if path.starts_with("$HOME") {
        path = home_path.join(path.strip_prefix("$HOME").unwrap());
    } else if path.starts_with("./") {
        path = current_dir.join(path.strip_prefix("./").unwrap());
    }

    Ok(path)
}

fn check_cerebra(config: &config::Config) {
    if !config.db_path.exists() {
        eprintln!("Database file does not exist. Please run `cerebra init` to create it.");
        std::process::exit(1);
    }

    // check if log file exists
    if !config.log_path.exists() {
        eprintln!("Log file does not exist. Please run `cerebra init` to create it.");
        std::process::exit(1);
    }

    // check if directories exist
    for path in &[
        &config.note_path,
        &config.misc_path,
        &config.todo_path,
        &config.journal_path,
        &config.code_path,
    ] {
        if !path.exists() {
            eprintln!("Directory does not exist: {}", path.display());
            eprintln!("Please run `cerebra init` to create it.");
            std::process::exit(1);
        }
    }
}
