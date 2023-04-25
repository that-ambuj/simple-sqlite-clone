use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

use anyhow::{bail, Result};

#[derive(Serialize, Deserialize)]
pub struct Row {
    id: u32,
    username: String,
    email: String,
}

impl Row {
    pub fn new(stmt: &str) -> Result<Self> {
        let split_stmt: Vec<_> = stmt.split_whitespace().collect();

        if let ["insert", id, username, email] = split_stmt.as_slice() {
            let id = id.parse::<u32>()?;

            return Ok(Self {
                id,
                username: (*username).into(),
                email: (*email).into(),
            });
        }

        bail!("Syntax Error(Invalid Syntax): {:?}", stmt);
    }

    pub fn write_to<T: Write>(&self, dest: T) -> Result<()> {
        bincode::serialize_into(dest, self)?;

        Ok(())
    }

    pub fn read_from<T: Read>(source: T) -> Result<Self> {
        let row = bincode::deserialize_from(source)?;

        Ok(row)
    }
}
