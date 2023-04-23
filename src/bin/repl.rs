use std::io::{self, BufWriter, Write};

#[derive(Debug)]
pub enum ExitCode {
    // OK,
    Failure,
    EOF,
    Error,
}

impl From<std::io::Error> for ExitCode {
    fn from(_: std::io::Error) -> Self {
        ExitCode::Error
    }
}

pub fn run() -> Result<(), ExitCode> {
    let mut stdout = BufWriter::new(io::stdout());
    stdout.write_all(b"db -> ")?;
    stdout.flush()?;

    let mut input = String::new();

    if io::stdin().read_line(&mut input).unwrap() > 0 {
        match input.as_ref() {
            ".exit\n" => {
                println!("Finished");
                return Ok(());
            }
            _ => {
                eprintln!("Unrecognized command: {}", input);
                return Err(ExitCode::Failure);
            }
        }
    } else {
        return Err(ExitCode::EOF);
    };
}

fn main() {
    run().unwrap();
}
