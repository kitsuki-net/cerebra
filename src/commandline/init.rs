use std::error::Error;
use std::path::Path;

use crate::config;
use crate::database;

pub fn init(config_path: &Path, config: &config::Config) -> Result<(), Box<dyn Error>> {
    config::create_config(config_path, &config).expect("Failed to create config file");

    create_paths(&config).expect("Failed to create directories");

    if !config.log_path.exists() {
        std::fs::File::create(&config.log_path)?;
        println!("Created log file: {}", config.log_path.display());
    } else {
        println!("Log file already exists: {}", config.log_path.display());
    }

    if !config.db_path.exists() {
        database::init::create_db_tables(&config.db_path)
            .expect("Failed to create database tables");
        println!("Created database: {}", config.db_path.display());
    } else {
        println!("Database already exists: {}", config.db_path.display());
    }

    Ok(())
}

fn create_paths(config: &config::Config) -> Result<(), Box<dyn Error>> {
    let paths = [
        config.note_path.clone(),
        config.misc_path.clone(),
        config.todo_path.clone(),
        config.journal_path.clone(),
        config.code_path.clone(),
    ];

    for path in paths.iter() {
        if !path.exists() {
            std::fs::create_dir_all(path).unwrap_or_else(|err| {
                eprintln!("Failed to create directory {}: {}", path.display(), err);
                std::process::exit(1);
            });
            println!("Created directory: {}", path.display());
            continue;
        }

        println!("Directory already exists: {}", path.display());
    }

    Ok(())
}
