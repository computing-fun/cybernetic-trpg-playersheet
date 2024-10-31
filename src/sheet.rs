use serde::{Deserialize, Serialize};

use crate::archive::{Archivable, Archive};

/// `Catalog` wraps different types of `Archive`, providing easy access to
/// specific `Archivable` types based on file extension.
#[derive(Debug, Clone)]
pub enum Catalog {
    Character(Archive<Character>),
    Race(Archive<Race>),
    Class(Archive<Class>),
}

impl Catalog {
    /// Attempts to look up a path and return the appropriate `Catalog`
    /// based on the file extension. Returns [`None`] if the extension does not match.
    pub fn lookup<P>(path: P) -> Option<Self>
    where
        P: AsRef<std::path::Path>,
    {
        match path
            .as_ref()
            .extension()
            .and_then(|os_str| os_str.to_str())?
        {
            Character::EXTENSION => Some(Catalog::Character(Archive::new(path)?)),
            Race::EXTENSION => Some(Catalog::Race(Archive::new(path)?)),
            Class::EXTENSION => Some(Catalog::Class(Archive::new(path)?)),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub name: String,
    pub race: Race,
    pub class: Class,
}

impl Archivable for Character {
    const EXTENSION: &'static str = "character-sheet";
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            race: Default::default(),
            class: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Race {
    pub name: String,
}

impl Archivable for Race {
    const EXTENSION: &'static str = "race-sheet";
}

impl Default for Race {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Class {
    pub name: String,
}

impl Archivable for Class {
    const EXTENSION: &'static str = "class-sheet";
}

impl Default for Class {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
        }
    }
}
