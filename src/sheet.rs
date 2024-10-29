use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Parse(serde_json::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(error) => error.fmt(f),
            Error::Parse(error) => error.fmt(f),
        }
    }
}

impl Error {
    pub fn not_selected() -> Self {
        Error::IO(std::io::Error::new(
            std::io::ErrorKind::Interrupted,
            "No file was selected",
        ))
    }
}

pub trait Sheet {
    const EXTENSION: &'static str;
}

impl<T> SheetWritable for T where T: Sheet + Serialize {}
pub trait SheetWritable: Sheet + Serialize {
    fn write(&self, path: &mut std::path::PathBuf) -> Result<(), Error> {
        path.set_extension(Self::EXTENSION);
        let file = std::fs::File::create(path).map_err(Error::IO)?;
        serde_json::to_writer(file, self).map_err(Error::Parse)
    }
}

impl<T> SheetReadable for T where T: Sheet + DeserializeOwned {}
pub trait SheetReadable: Sheet + DeserializeOwned {
    fn read(path: &mut std::path::PathBuf) -> Result<Self, Error> {
        path.set_extension(Self::EXTENSION);
        let file = std::fs::File::open(path).map_err(Error::IO)?;
        serde_json::from_reader(file).map_err(Error::Parse)
    }
}

impl<T> SheetCreatable for T where T: SheetWritable + Default {}
pub trait SheetCreatable: SheetWritable + Default {
    fn create(path: &mut std::path::PathBuf) -> Result<Self, Error> {
        let sheet = Self::default();
        sheet.write(path)?;
        Ok(sheet)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub name: String,
    pub race: Race,
    pub class: Class,
}

impl Sheet for Character {
    const EXTENSION: &'static str = "playersheet";
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: "Hero".to_string(),
            race: Default::default(),
            class: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Race {
    pub name: String,
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

impl Default for Class {
    fn default() -> Self {
        Self {
            name: "Classless".to_string(),
        }
    }
}
