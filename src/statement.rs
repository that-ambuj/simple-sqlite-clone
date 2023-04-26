use crate::Row;
use anyhow::{bail, Result};

pub enum Statement {
    Insert(Row),
    Select,
}

impl Statement {
    pub fn prepare(stmt: &str) -> Result<Self> {
        if stmt.len() < 6 {
            bail!("Unrecognized keyword at start of {:?}", &stmt);
        }

        let (start, _) = stmt.split_at(6);

        match start {
            "insert" => Ok(Self::Insert(Row::from_str(stmt)?)),
            "select" => Ok(Self::Select),
            _ => bail!("Unrecognized keyword at start of {:?}", stmt),
        }
    }

    pub fn execute(&self) -> Result<()> {
        match self {
            Statement::Insert(row) => {
                println!("This is where you would do an insert.");
                Ok(())
            }
            Statement::Select => {
                println!("This is where you would do a select.");
                Ok(())
            }
        }
    }
}
