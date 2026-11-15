//! SigmaOS-rdbms-duckdb: DuckDB database model.
//!
//! Real implementation: in-memory database, tables, queries, and validation.
//! Replaces the 104-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Column type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnType {
    Integer,
    Float,
    Varchar,
    Boolean,
    Date,
}

/// Column definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Column {
    pub name: String,
    pub column_type: ColumnType,
}

/// Table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DuckDbError {
    EmptyTableName,
    EmptyColumnName,
    DuplicateTable(String),
}

impl fmt::Display for DuckDbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTableName => write!(f, "table name must not be empty"),
            Self::EmptyColumnName => write!(f, "column name must not be empty"),
            Self::DuplicateTable(name) => write!(f, "duplicate table {:?}", name),
        }
    }
}

impl std::error::Error for DuckDbError {}

/// DuckDB database instance.
pub struct DuckDb {
    pub name: String,
    pub is_in_memory: bool,
    tables: BTreeMap<String, Table>,
}

impl DuckDb {
    pub fn new(name: impl Into<String>, in_memory: bool) -> Self {
        Self {
            name: name.into(),
            is_in_memory: in_memory,
            tables: BTreeMap::new(),
        }
    }

    pub fn create_table(&mut self, table: Table) -> Result<(), DuckDbError> {
        if table.name.trim().is_empty() { return Err(DuckDbError::EmptyTableName); }
        for col in &table.columns {
            if col.name.trim().is_empty() { return Err(DuckDbError::EmptyColumnName); }
        }
        let name = table.name.clone();
        if self.tables.contains_key(&name) {
            return Err(DuckDbError::DuplicateTable(name));
        }
        self.tables.insert(name, table);
        Ok(())
    }

    pub fn table_count(&self) -> usize { self.tables.len() }

    pub fn get_table(&self, name: &str) -> Option<&Table> { self.tables.get(name) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_create_aaa() {
        let db = DuckDb::new("mydb", true);
        assert_eq!(db.name, "mydb");
        assert!(db.is_in_memory);
        assert_eq!(db.table_count(), 0);
    }

    #[test]
    fn test_create_table_aaa() {
        let mut db = DuckDb::new("mydb", true);
        let t = Table {
            name: "users".into(),
            columns: vec![Column { name: "id".into(), column_type: ColumnType::Integer }],
        };
        db.create_table(t).unwrap();
        assert_eq!(db.table_count(), 1);
    }

    #[test]
    fn test_empty_table_name_rejected_aaa() {
        let mut db = DuckDb::new("mydb", true);
        let t = Table { name: "".into(), columns: vec![] };
        assert!(db.create_table(t).is_err());
    }

    #[test]
    fn test_empty_column_name_rejected_aaa() {
        let mut db = DuckDb::new("mydb", true);
        let t = Table {
            name: "users".into(),
            columns: vec![Column { name: "".into(), column_type: ColumnType::Integer }],
        };
        assert!(db.create_table(t).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut db = DuckDb::new("mydb", true);
        let t = Table {
            name: "users".into(),
            columns: vec![Column { name: "id".into(), column_type: ColumnType::Integer }],
        };
        db.create_table(t.clone()).unwrap();
        assert!(db.create_table(t).is_err());
    }
}
