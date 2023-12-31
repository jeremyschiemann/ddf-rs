use rusqlite::{Connection, Result};
use std::io;
use strum::IntoEnumIterator;

mod command;
mod question;
mod stat;
mod to_db;

use command::Command;

fn main() -> Result<()> {
    let db_path = "./test.db";

    let conn = match Connection::open(db_path) {
        Ok(conn) => conn,
        Err(error) => panic!("Error opening database: {}", error),
    };

    let mut user_input = String::new();

    loop {
        eprint!("Command: ");
        user_input.clear();

        if let Err(error) = io::stdin().read_line(&mut user_input) {
            panic!("Input error: {}", error);
        }

        let trimmed_input = user_input.trim();
        let command = Command::from_string(trimmed_input);

        match command {
            Command::Quit => break,
            Command::Stat => {
                let (seen, seen_since_reset, len_seen) = stat::query_metadata(&conn)?;
                stat::print_stat(seen, seen_since_reset, len_seen);
            }
            Command::Reload => match to_db::reload_from_files(&conn) {
                Ok(_) => println!("Reloaded from files"),
                Err(error) => println!("Error reloading from files: {}", error),
            },
            Command::ResetSeen => question::reset_seen(&conn)?,
            Command::ShowAll => question::show_all(&conn)?,
            Command::NextQuestion => {
                let next_question = question::query_next_question(&conn)?;
                question::print_question(&next_question);
                question::update_question_meta(&conn, &next_question)?;
            }
            Command::Help => {
                println!("Available commands:");
                for variant in Command::iter() {
                    match variant {
                        Command::Other(_) => continue,
                        _ => println!("\t- {} ", variant),
                    }
                }
            }
            Command::Other(cmd) => {
                println!("Unknown command: {}", cmd);
                println!("Available commands:");
                for variant in Command::iter() {
                    match variant {
                        Command::Other(_) => continue,
                        _ => println!("\t- {} ", variant),
                    }
                }
            }
        }
    }

    Ok(())
}
