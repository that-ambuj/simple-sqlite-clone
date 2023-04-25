use anyhow::{bail, Result};

pub struct Statement {
    stmt_type: StatementType,
}

pub enum StatementType {
    Insert,
    Select,
}

impl Statement {
    pub fn prepare(stmt: &str) -> Result<Self> {
        if stmt.len() < 6 {
            bail!("Unrecognized keyword at start of {:?}", &stmt);
        }

        let (start, _) = stmt.split_at(6);

        match start {
            "insert" => Ok(Self {
                stmt_type: StatementType::Insert,
            }),
            "select" => Ok(Self {
                stmt_type: StatementType::Select,
            }),
            _ => bail!("Unrecognized keyword at start of {:?}", stmt),
        }
    }

    pub fn execute(&self) -> Result<()> {
        match self.stmt_type {
            StatementType::Insert => {
                println!("This is where you would do an insert.");
                Ok(())
            }
            StatementType::Select => {
                println!("This is where you would do a select.");
                Ok(())
            }
        }
    }
}
