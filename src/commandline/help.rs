pub fn _help() {
    println!("Usage: cerebra <command> [<args>]");
    println!();
    println!("Commands:");
    println!("  init               Initialize Cerebra");
    println!("  add                Add a new entry to the base");
    println!("  rm                 Remove an entry from the database");
    println!("  mod                Modify an entry in the database");
    println!("  config             Configure the database");
    println!("  sync               Sync the database");
    println!("  draw               Draw the database");
}

// TODO: implement database json initialization

pub fn _add() {
    println!("Usage: cerebra add <type> <entry> [<tags>]");
    println!();
    println!("Add a new entry to Cerebra");
    println!();
    println!("Arguments:");
    println!("  type               The type of entry to add to the database, e.g. 'note'");
    println!("  entry              The entry to add to the database");
    println!("  tags               The tags to associate with the entry, e.g 'project:foo'");
    println!("");
    println!("Examples:");
    println!("  cerebra add note new note");
    println!("  cerebra add note new note project:foo");
}

pub fn _remove() {
    println!("Usage: cerebra rm <id>");
    println!();
    println!("Remove an entry from Cerebra");
    println!();
    println!("Arguments:");
    println!("  id                 The id of the entry to remove from the database");
    println!("");
    println!("Examples:");
    println!("  cerebra rm 1");
}

pub fn _mod_entry() {
    println!("Usage: cerebra mod <id> [<entry>] [<tags>]");
    println!();
    println!("Modify an entry in Cerebra");
    println!();
    println!("Arguments:");
    println!("  id                 The id of the entry to modify in the database");
    println!("  entry              The new entry to replace the old entry with");
    println!("  tags               The new tags to replace the old tags with");
    println!("");
    println!("Examples:");
    println!("  cerebra mod 1 new entry");
    println!("  cerebra mod 1 new entry project:foo");
    println!("  cerebra mod 1 project:foo");
}

pub fn _config() {
    println!("Usage: cerebra config <key> <value>");
    println!();
    println!("Configure Cerebra");
    println!();
    println!("Arguments:");
    println!("  key                The key to configure, e.g. 'db_path'");
    println!("  value              The value to set the key to, e.g. 'cerebra.db'");
    println!("");
    println!("Examples:");
    println!("  cerebra config db_path cerebra.db");
}

pub fn _sync() {
    println!("Usage: cerebra sync [<type>]");
    println!();
    println!("Sync the database with a remote server");
    println!();
    println!("Arguments:");
    println!("  type               The type of sync to perform, can be 'rsync' or 'git'");
}

pub fn _draw() {
    println!("Usage: cerebra draw");
    println!();
    println!("Draw the database");
}

// TODO: tutorial
pub fn _intro() {
    println!("Usage: cerebra intro");
    println!();
    println!("Print the introduction to Cerebra");
}
