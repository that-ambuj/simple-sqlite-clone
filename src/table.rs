use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

use crate::Row;

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Table {
    rows: Vec<Row>,
}

impl Table {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, row: Row) {
        self.rows.push(row);
    }

    pub fn select_all(&self) {
        for row in &self.rows {
            row.print();
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let bytes = bincode::serialize(self)?;

        Ok(bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let table = bincode::deserialize(bytes)?;

        Ok(table)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_to_and_from_bytes() {
        let mut table = Table::new();

        table.push(Row::new(1, "user1", "user1@email.com"));
        table.push(Row::new(2, "user2", "user2@email.com"));

        let bytes = table.to_bytes();
        assert!(bytes.is_ok());

        let bytes = bytes.unwrap();

        let new_table = Table::from_bytes(&bytes);
        assert!(new_table.is_ok());
        assert_eq!(table, new_table.unwrap());
    }
}
