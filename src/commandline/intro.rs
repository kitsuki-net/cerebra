use std::error::Error;

pub fn intro() -> Result<(), Box<dyn Error>> {
    let intro = r#"
Welcome to Cerebra!
~~~~~~~~~~~~~~~~~~~

Cerebra is a terminal-based knowledge management tool written in Rust.

It allows you to store and manage notes, tasks, and other information in a simple and efficient manner.

Designed with ease of use, flexibility and customization in mind, it's a lightweight and powerful tool that can adapt to your workflow.

This tool uses Markdown files to store information while a database takes care of the structure, metadata and relationships between entries.

Key Features
============

- Store notes, tasks, and other information in a SQLite database and Markdown files
- Organize entries with tags and categories
- Customize the behavior of Cerebra with a configuration file
- Use a simple and intuitive command-line interface
- Use a TUI for a more interactive experience
- Sync your database with other devices using Git or rsync
- Draw a graph of your database to visualize your knowledge base structure

Getting Started
===============
To get started, you can use `cerebra init` to...
- Initialize a SQLite database to store your notes and tasks
- Set up a configuration file to customize the behavior of Cerebra
- Create a log file to keep track of changes to your database
- Define paths to directories for notes, todos, and other files

To see all the possible actions, you can run `cerebra --help` or `cerebra -h`.

Configuration File
==================
Cerebra searches for the configuration file in the following locations, in order:
- $HOME/.cerebra/cerebra.conf
- $HOME/.cerebra.conf
- ./.cerebra.conf

The default location is $HOME/cerebra/.cerebra.conf.

The configuration file can contain the following keys:
- db_path: path to the SQLite database file
- log_path: path to the log file
- note_path: path to the directory where notes are stored
- misc_path: path to the directory where other files are stored
- template_path: path to the directory where templates are stored
- todo_path: path to the directory where todos are stored
- journal_path: path to the directory where journal entries are stored
- code_path: path to the directory where code snippets are stored
- theme: theme to use for the TUI
- editor: editor to use for editing entries

"#;

    println!("{}", intro);
    Ok(())
}
