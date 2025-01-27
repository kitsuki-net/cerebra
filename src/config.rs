use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub db_path: PathBuf,
    pub log_path: PathBuf,
    pub note_path: PathBuf,
    pub misc_path: PathBuf,
    pub todo_path: PathBuf,
    pub journal_path: PathBuf,
    pub code_path: PathBuf,
    pub theme: String,
    pub editor: String,
}

impl Config {
    pub fn new(
        db_path: PathBuf,
        log_path: PathBuf,
        note_path: PathBuf,
        misc_path: PathBuf,
        todo_path: PathBuf,
        journal_path: PathBuf,
        code_path: PathBuf,
        theme: String,
        editor: String,
    ) -> Config {
        Config {
            db_path,
            log_path,
            note_path,
            misc_path,
            todo_path,
            journal_path,
            code_path,
            theme,
            editor,
        }
    }

    pub fn default() -> Config {
        let home = std::env::var("HOME").expect("Could not get home directory");
        let home_dir = PathBuf::from(home);

        Config {
            db_path: home_dir.join(".cerebra/cerebra.db"),
            log_path: home_dir.join(".cerebra/cerebra.log"),
            note_path: home_dir.join("cerebra/notes"),
            misc_path: home_dir.join("cerebra/misc"),
            todo_path: home_dir.join("cerebra/todos"),
            journal_path: home_dir.join("cerebra/code"),
            code_path: home_dir.join("cerebra/code"),
            theme: "dark".to_string(),
            editor: "nvim".to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            r#"db_path={}
log_path={}
note_path={}
misc_path={}
todo_path={}
journal_path={}
code_path={}
theme={}
editor={}
"#,
            self.db_path.to_str().unwrap(),
            self.log_path.to_str().unwrap(),
            self.note_path.to_str().unwrap(),
            self.misc_path.to_str().unwrap(),
            self.todo_path.to_str().unwrap(),
            self.journal_path.to_str().unwrap(),
            self.code_path.to_str().unwrap(),
            self.theme,
            self.editor,
        )
    }

    pub fn from_string(config_str: &str) -> Config {
        let mut config = Config::default();

        for line in config_str.lines() {
            let mut parts = line.split('=');
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();

            match key {
                "db_path" => config.db_path = PathBuf::from(value),
                "log_path" => config.log_path = PathBuf::from(value),
                "note_path" => config.note_path = PathBuf::from(value),
                "misc_path" => config.misc_path = PathBuf::from(value),
                "todo_path" => config.todo_path = PathBuf::from(value),
                "journal_path" => config.journal_path = PathBuf::from(value),
                "code_path" => config.code_path = PathBuf::from(value),
                "theme" => config.theme = value.to_string(),
                "editor" => config.editor = value.to_string(),
                _ => panic!("Invalid key in config file"),
            }
        }

        config
    }
}

pub fn get_config_path() -> Option<PathBuf> {
    let home = std::env::var("HOME").expect("Could not get home directory");
    let home_dir = PathBuf::from(home);
    let current_dir = std::env::current_dir().expect("Could not get current directory");

    let config_paths = [
        home_dir.join(".cerebra/cerebra.conf"),
        home_dir.join(".cerebra.conf"),
        current_dir.join(".cerebra.conf"),
    ];

    Some(config_paths.iter().find(|&path| path.exists()).cloned()?)
}

pub fn create_config(config_path: &Path, config: &Config) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent).expect("Could not create config directory");
    }

    // if config exists then prompt if overwrite
    if config_path.exists() {
        println!(
            "Config file already exists at {}",
            config_path.to_str().unwrap()
        );

        let mut input = String::new();
        println!("Do you want to overwrite it? (y/n)");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        if input.trim() != "y" {
            println!("Skipping config file creation");
            return Ok(());
        }
    }

    let mut file = File::create(config_path)
        .unwrap_or_else(|_| File::create(config_path).expect("Could not create config file"));

    file.write_all(config.to_string().as_bytes())
        .expect("Could not write to config file");

    println!("Config file created at {}", config_path.to_str().unwrap());

    Ok(())
}

pub fn read_config(config_path: &Path) -> Result<Config, Box<dyn Error>> {
    let config_str = std::fs::read_to_string(&config_path).expect("Could not read config file");

    Ok(Config::from_string(&config_str))
}
