pub mod utils;

use std::{error::Error, io};

use colorized::{colorize_println, Colors};
use rusqlite::params;
use utils::{FlagArgs, NonFlagArgs, Todo};

use crate::utils::{get_db_conn, unwrap_arg_or_quit};
static HELP_TEXT: &str = "
        taskly [COMMAND] [OPTIONS] [ARGS]

        OPTIONS:
            -c          Show only completed todos
            -p          Show only pending todos

        COMMANDS:
            help        Show this message
            add         Add a new todo
            edit        Edit an existing todo
            list        List all todos
            remove      Remove an existing todo
            done        Mark a todo as done
            undone      Mark a todo as undone
            clear       Remove all todos
            
        ARGS:
            The id of the todo to be edited, removed or marked as done/undone
        ";
pub struct Conditions<'a> {
    pub flag: FlagArgs,
    pub non_flag: NonFlagArgs<'a>,
}
impl Conditions<'_> {
    pub fn new() -> Self {
        Self {
            flag: FlagArgs::All,
            non_flag: NonFlagArgs::Help,
        }
    }
    pub fn from<'a>(
        non_flag_arg: Option<&'a str>,
        arg2: Option<&'a str>,
        arg3: Option<&'a str>,
    ) -> Conditions<'a> {
        let (flag_arg, input) = match (arg2, arg3) {
            (Some(arg2), Some(arg3)) => {
                if arg2.starts_with('-') {
                    (Some(arg2), Some(arg3))
                } else {
                    (Some(arg3), Some(arg2))
                }
            }
            (Some(arg2), None) => {
                if arg2.starts_with('-') {
                    (Some(arg2), arg3)
                } else {
                    (arg3, Some(arg2))
                }
            }
            _ => (arg2, arg3),
        };
        //write docs for this
        let non_flag: NonFlagArgs<'a> = match non_flag_arg {
            Some("help") => NonFlagArgs::Help,
            Some("add") => NonFlagArgs::Add(input),
            Some("edit") => NonFlagArgs::Edit(input),
            Some("list") => NonFlagArgs::List,
            Some("remove") | Some("rm") => NonFlagArgs::Remove(input),
            Some("done") => NonFlagArgs::Done(input),
            Some("undone") => NonFlagArgs::Undone(input),
            Some("clear") => NonFlagArgs::Clear,
            _ => NonFlagArgs::Help,
        };
        let flag = match flag_arg {
            Some("-c") => FlagArgs::CompletedOnly,
            Some("-p") => FlagArgs::UncompletedOnly,
            _ => FlagArgs::All,
        };
        Conditions { flag, non_flag }
    }
    pub fn exec(&self) -> Result<(), Box<dyn Error>> {
        if let Conditions {
            flag: _,
            non_flag: NonFlagArgs::Help,
        } = self
        {
            println!("{}", HELP_TEXT);
            return Ok(());
        }
        let conn = get_db_conn()?;
        match self {
            Conditions {
                flag: _,
                non_flag: NonFlagArgs::Clear,
            } => {
                conn.execute("DROP TABLE todo", ())?;
                Ok(())
            }
            Conditions {
                flag: _,
                non_flag: NonFlagArgs::Add(value),
            } => {
                let todo = unwrap_arg_or_quit(value);
                conn.execute("INSERT INTO todo (NAME) VALUES ($1)", params![todo])?;
                Ok(())
            }
            Conditions {
                flag: _,
                non_flag: NonFlagArgs::Remove(value),
            } => {
                let todo = unwrap_arg_or_quit(value);
                conn.execute("DELETE FROM todo WHERE id=$1", params![todo])?;
                Ok(())
            }
            Conditions {
                flag: _,
                non_flag: NonFlagArgs::Edit(value),
            } => {
                let todo = unwrap_arg_or_quit(value);
                colorize_println("Enter new name for this todo", Colors::BrightMagentaFg);
                let mut user_input = String::new();
                let stdin = io::stdin(); // We get `Stdin` here.
                stdin.read_line(&mut user_input)?;
                conn.execute(
                    "UPDATE todo set name=$1 WHERE id=$2",
                    params![user_input.trim(), todo],
                )?;
                Ok(())
            }
            Conditions {
                flag: _,
                non_flag: NonFlagArgs::Done(value),
            } => {
                let todo = unwrap_arg_or_quit(value);
                conn.execute(
                    "UPDATE todo set status='COMPLETED' WHERE id=$1",
                    params![todo],
                )?;
                Ok(())
            }
            Conditions {
                flag: _,
                non_flag: NonFlagArgs::Undone(value),
            } => {
                let todo = unwrap_arg_or_quit(value);
                conn.execute("UPDATE todo set status='TODO' WHERE id=$1", params![todo])?;
                Ok(())
            }
            Conditions {
                flag,
                non_flag: NonFlagArgs::List,
            } => {
                let (status, query_extra_params) = match flag {
                    FlagArgs::All => ("", ""),
                    FlagArgs::CompletedOnly => ("Completed", " WHERE status='COMPLETED'"),
                    FlagArgs::UncompletedOnly => ("Pending", " WHERE status='TODO'"),
                };
                let query = String::from("SELECT id,name,status FROM todo") + query_extra_params;
                let mut stmt = conn.prepare(&query)?;
                let todo_iter = stmt.query_map([], |row| {
                    Ok(Todo {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        status: row.get(2)?,
                    })
                })?;
                let title = String::from("Here are all the ") + status + " todos";
                colorize_println(title, Colors::BrightGreenFg);
                colorize_println(
                    format!("{0: <5}  {1: <20}  {2: <9}", "ID", "NAME", "STATUS"),
                    Colors::BrightBlueFg,
                );
                for todo in todo_iter {
                    println!("{}", todo?);
                }
                Ok(())
            }
            _ => {
                println!("{}", HELP_TEXT);
                Ok(())
            }
        }
    }
}

impl Default for Conditions<'_> {
         fn default() -> Self {
             Self::new()
         }
}