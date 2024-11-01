use std::{error::Error, fmt::Display, path::Path};

use redb::{
    CommitError, Database, DatabaseError, StorageError, TableDefinition, TableError,
    TransactionError,
};
use serde::{de::DeserializeOwned, Serialize};

pub struct Book {
    content: Database,
}

impl Book {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DatabaseError> {
        Ok(Self {
            content: Database::create(path)?,
        })
    }

    pub fn read<S>(&self, name: &str) -> Result<S, BookError>
    where
        S: Sheet,
    {
        let rt = self.content.begin_read()?;
        let table = rt.open_table(S::TABLE_DEFINITION)?;
        let json = table
            .get(name.to_string())?
            .ok_or(BookError::NotFound(name.to_string()))?
            .value();
        let sheet = serde_json::from_str(&json)?;
        Ok(sheet)
    }

    pub fn write<S>(&self, sheet: &S) -> Result<(), BookError>
    where
        S: Sheet,
    {
        let json = serde_json::to_string(sheet)?;
        let wt = self.content.begin_write()?;
        {
            let mut table = wt.open_table(S::TABLE_DEFINITION)?;
            table.insert(sheet.name(), json)?;
        }
        wt.commit()?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum BookError {
    Database(DatabaseError),
    Transaction(TransactionError),
    Table(TableError),
    Storage(StorageError),
    NotFound(String),
    Parse(serde_json::Error),
    Commit(CommitError),
}

impl Error for BookError {}

impl Display for BookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BookError::Database(err) => write!(f, "Database error: {}", err),
            BookError::Transaction(err) => write!(f, "Transaction error: {}", err),
            BookError::Table(err) => write!(f, "Table error: {}", err),
            BookError::Storage(err) => write!(f, "Storage error: {}", err),
            BookError::NotFound(err) => write!(f, "Not Found error: {}", err),
            BookError::Parse(err) => write!(f, "Parse error: {}", err),
            BookError::Commit(err) => write!(f, "Commit error: {}", err),
        }
    }
}

impl From<DatabaseError> for BookError {
    fn from(value: DatabaseError) -> Self {
        BookError::Database(value)
    }
}

impl From<TransactionError> for BookError {
    fn from(value: TransactionError) -> Self {
        BookError::Transaction(value)
    }
}

impl From<TableError> for BookError {
    fn from(value: TableError) -> Self {
        BookError::Table(value)
    }
}

impl From<StorageError> for BookError {
    fn from(value: StorageError) -> Self {
        BookError::Storage(value)
    }
}

impl From<serde_json::Error> for BookError {
    fn from(value: serde_json::Error) -> Self {
        BookError::Parse(value)
    }
}

impl From<CommitError> for BookError {
    fn from(value: CommitError) -> Self {
        BookError::Commit(value)
    }
}

pub trait Sheet: Serialize + DeserializeOwned {
    const TYPE: &'static str;
    const TABLE_DEFINITION: TableDefinition<'static, String, String> =
        TableDefinition::new(Self::TYPE);
    fn name(&self) -> String;
}
