use std::{error::Error, fmt::Display, path::Path};

use non_empty_string::NonEmptyString;
use redb::{
    CommitError, Database, DatabaseError, ReadableTable, StorageError, TableDefinition, TableError,
    TransactionError,
};
use serde::{de::DeserializeOwned, Serialize};

pub struct Book {
    name: String,
    content: Database,
}

impl Book {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DatabaseError> {
        Ok(Self {
            name: path
                .as_ref()
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            content: Database::create(path)?,
        })
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn read<S>(&self, name: &str) -> Result<S, BookError>
    where
        S: Sheet,
    {
        Ok(serde_json::from_str(
            &self
                .content
                .begin_read()?
                .open_table(S::TABLE_DEFINITION)?
                .get(name.to_string())?
                .ok_or(BookError::NotFound(name.to_string()))?
                .value(),
        )?)
    }

    pub fn write<S>(&self, sheet: &S) -> Result<(), BookError>
    where
        S: Sheet,
    {
        let wt = self.content.begin_write()?;
        wt.open_table(S::TABLE_DEFINITION)?
            .insert(sheet.name(), serde_json::to_string(sheet)?)?;
        wt.commit()?;
        Ok(())
    }

    pub fn table_of_contents<S>(&self) -> Result<Vec<Result<String, BookError>>, BookError>
    where
        S: Sheet,
    {
        Ok(self
            .content
            .begin_read()?
            .open_table(S::TABLE_DEFINITION)?
            .iter()?
            .map(|sheet| sheet.map(|ok| ok.0.value()).map_err(BookError::Storage))
            .collect())
    }

    pub fn section<S>(&self) -> Result<Vec<Result<S, BookError>>, BookError>
    where
        S: Sheet,
    {
        Ok(self
            .content
            .begin_read()?
            .open_table(S::TABLE_DEFINITION)?
            .iter()?
            .map(|table_access| {
                table_access
                    .map(|ok| ok.1.value())
                    .map_err(BookError::Storage)
            })
            .map(|table_value| {
                table_value.and_then(|data| serde_json::from_str(&data).map_err(BookError::Parse))
            })
            .collect::<Vec<Result<S, BookError>>>())
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
    fn name(&self) -> NonEmptyString;
}
