# Cerebra

Cerebra is a terminal-based knowledge management tool written in Rust.

It allows you to store and manage notes with simple and efficient commands.

Designed with ease of use, flexibility and customization in mind, it's a lightweight and powerful tool that can adapt to your workflow.

This tool uses Markdown files to store information while a database takes care of the structure, metadata and relationships between entries.

## Key Features

- Store notes, tasks, and other information in a SQLite database and Markdown files
- Customize the behavior of Cerebra with a configuration file
- Use a simple and intuitive command-line interface to interact with your knowledge base
- Use a TUI for a more interactive experience

## Planned Features

- [ ] Besides notes, one should be able to store
  - [ ] _tasks/projects_: todos, deadlines, etc.
  - [ ] _ideas_: concepts, theories, etc.
  - [ ] _writings_: journals, stories, essays, etc.
  - [ ] _misc/sources_: PDFs, images, videos, etc.
  - [ ] _code_: scripts, snippets, etc.
- [ ] A dynamic dependence between database and files: If something in the files is changed, the database should be updated and vice versa (probably needs a watcher and will be pretty difficult😅)
- [ ] Better error handling and logging
- [ ] A performant search based on tags, categories, and content
- [ ] A taskwarrior-like view of displaying the different types of entries
- [ ] A draw view of the database that visualizes the connections between tables
- [ ] A draw view of the database that visualizes the connections between entries
- [ ] More theming configuration options

## Getting Started

### Installation

To install Cerebra, you can clone the repository and build it yourself:

```bash
git clone https://github.com/kitsuki-net/cerebra.git
cd cerebra
cargo build --release
```

After building the project, you can run the binary with `./target/release/cerebra`, `cargo run --release` or add it to your PATH.

### Usage

To get started, you can use `cerebra init` to...

- Initialize a SQLite database to store your notes and tasks
- Set up a configuration file to customize the behavior of Cerebra
- Create a log file to keep track of changes to your database
- Define paths to directories for notes, todos, and other files

After initializing your database, you can start adding entries with `cerebra add`.

```bash
cerebra add note "This is a note" source:book topic:science/physics context:school
```

=> This adds an entry to the database while also creating the science folder and creating a physics.md file inside it with the note content.

To see all the possible actions, you can run `cerebra --help` or `cerebra -h`.

## Inspired by

- [Obsidian](https://obsidian.md/): a powerful knowledge base that uses Markdown files to store notes and a graph view to visualize connections between them
- [Taskwarrior](https://taskwarrior.org/): a flexible task manager that uses a command-line interface and a SQLite database to interact with tasks and projects

This project was intended to replace my primary use of Obsidian as knowledge management tool. I wanted a lightweight, terminal-based tool that could be easily customized and extended to fit my workflow.

## Contributing

Since this is a personal project of mine, this is a tool that is tailored to my needs and is pretty much opinionated. However, I am open to suggestions and improvements that could benefit other users as well :)

Anyone who wants to contribute is welcome to do so, and I will be happy to help you get started with the codebase or answer any questions you may have.

This is my first time coding in Rust, so I'm sure there are many things that could be improved in the codebase. So, if you have any ideas, suggestions, or bug reports, feel free to open an issue or a pull request! You can also contact me at [contact@kitsuki.net](mailto:contact@kitsuki.net).

I can't guarantee that I will be able to respond to all e-mails or merge all pull requests in a reasonable amount of time, but I will do my best to keep the project up to date and maintain it.

Thank you for your interest in Cerebra! I hope you find it useful and enjoy using it as much as I do <3
