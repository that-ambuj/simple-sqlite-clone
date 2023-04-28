use crate::{Row, Table};
use anyhow::{bail, Result};

pub enum Statement {
    Insert(Row),
    Select,
}

impl Statement {
    /// This function parses a given string and return a `Statement`
    /// in an Ok() variant if the given string is parsed correctly.
    /// Otherwise, It returns an error Err.
    pub fn prepare(stmt: &str) -> Result<Self> {
        if stmt.len() < 6 {
            bail!("Unrecognized keyword at start of {:?}", &stmt);
        }

        let (start, _) = stmt.split_at(6);

        match start {
            "insert" => Ok(Self::Insert(Row::from_string(stmt)?)),
            "select" => Ok(Self::Select),
            _ => bail!("Syntax Error. Could not parse statement: {:?}", stmt),
        }
    }

    /// This function takes a `Table` struct and executes the given
    /// `Statement` on the table.
    pub fn execute(&self, table: &mut Table) -> Result<()> {
        match self {
            Statement::Insert(row) => {
                table.push(row.clone());
                Ok(())
            }
            Statement::Select => {
                table.select_all();
                Ok(())
            }
        }
    }
}
