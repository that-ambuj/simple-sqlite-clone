use serde::{Deserialize, Serialize};

use anyhow::{bail, Result};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Row {
    id: u32,
    username: String,
    email: String,
}

impl Row {
    pub fn new(id: u32, username: &str, email: &str) -> Self {
        Self {
            id,
            username: username.into(),
            email: email.into(),
        }
    }

    pub fn from_string(stmt: &str) -> Result<Self> {
        let split_stmt: Vec<_> = stmt.split_whitespace().collect();

        if let ["insert", id, username, email] = split_stmt.as_slice() {
            let id = id.parse::<u32>()?;

            return Ok(Self::new(id, username, email));
        }

        bail!("Syntax Error(Invalid Syntax): {:?}", stmt);
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let bytes = bincode::serialize(&self)?;

        Ok(bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let row = bincode::deserialize(bytes)?;

        Ok(row)
    }

    pub(crate) fn print(&self) {
        println!("({}, {}, {})", self.id, self.username, self.email);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_correctly_from_string() {
        let row = Row::from_string("insert 1 ambuj example@email.com");

        assert!(row.is_ok());
        let row = row.unwrap();

        assert_eq!(1, row.id);
        assert_eq!("ambuj", row.username);
        assert_eq!("example@email.com", row.email);
    }

    #[test]
    fn converts_to_and_from_bytes() {
        let row = Row::new(1, "doejohn", "john@doe.com");

        let bytes = row.to_bytes();
        assert!(bytes.is_ok());

        let bytes = bytes.unwrap();

        let new_row = Row::from_bytes(&bytes);
        assert!(new_row.is_ok());
        assert_eq!(row, new_row.unwrap());
    }
}
