use anyhow::{anyhow, Result};
use std::{
    io::{self, Write},
    process::exit,
};

use simple_sqlite_clone::Statement;

#[derive(Debug)]
pub enum ExitCode {
    OK,
    Failure,
    EOF,
    Error,
}

pub fn run() -> Result<()> {
    let mut stdout = io::stdout();
    stdout.write_all(b"db -> ")?;
    stdout.flush()?;

    let mut input = String::new();

    if io::stdin().read_line(&mut input).unwrap() > 0 {
        if let (".", _) = input.split_at(1) {
            if let Err(e) = run_meta_command(&input) {
                eprintln!("{}", e);
                exit(ExitCode::Failure as i32)
            } else {
                println!("Executed.");
            }
        }

        let statement = Statement::prepare(&input)?;
        statement.execute()?;

        Ok(())
    } else {
        exit(ExitCode::EOF as i32);
    }
}

/// Meta Commands are the commands that don't actually
/// change any data in the database but are used for repl specific
/// interactions. They are basically Non-SQL commands.
/// For Example `.exit`
fn run_meta_command(cmd: &str) -> Result<()> {
    match cmd {
        ".exit\n" => {
            println!("Finished");
            exit(ExitCode::OK as i32)
        }
        _ => {
            return Err(anyhow!("Unrecognised command: {}", cmd));
        }
    }
}

fn main() {
    loop {
        let _ = run();
    }
}
