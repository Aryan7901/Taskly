use std::{
    error::Error,
    fmt::Display,
    fs::{self},
    process,
};

use colorized::{colorize_println, Colors};
use rusqlite::Connection;
pub enum FlagArgs {
    CompletedOnly,
    UncompletedOnly,
    All,
}
#[derive(Debug)]
pub enum NonFlagArgs<'a> {
    Help,
    List,
    Add(Option<&'a str>),
    Remove(Option<&'a str>),
    Edit(Option<&'a str>),
    Done(Option<&'a str>),
    Undone(Option<&'a str>),
    Clear,
}

pub struct Todo {
    pub id: isize,
    pub name: String,
    pub status: String,
}
impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{0: <5}  {1: <20}  {2: <9}",
            self.id, self.name, self.status
        ))
    }
}

pub fn unwrap_arg_or_quit<'a>(val: &'a Option<&'a str>) -> &'a str {
    val.unwrap_or_else(|| {
        colorize_println("Insufficient Arguments Provided!", Colors::RedFg);
        println!("Run todo help for information on commands");
        process::exit(1)
    })
}
pub fn get_db_conn() -> Result<Connection, Box<dyn Error>> {
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => {
            eprintln!("Failed to get home directory");
            process::exit(1);
        }
    };
    let directory_name = ".todo";
    let directory_path = home_dir.join(directory_name);
    fs::create_dir_all(&directory_path)?;
    let db_path = directory_path.join("todo.db");
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            status TEXT DEFAULT 'TODO' CHECK(status IN ('TODO', 'COMPLETED'))
        );",
        (),
    )?;
    Ok(conn)
}
