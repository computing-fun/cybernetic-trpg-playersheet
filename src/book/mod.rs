pub mod ability;
pub mod class;
mod lua_object;

use std::{
    fs::File,
    io::{self, Write},
    path::{Path, PathBuf},
};

use class::{Balance, CharacterClass, Class};
use lua_object::FromLuaChunk;
use non_empty_string::NonEmptyString;
use serde::{Deserialize, Serialize};
use zip::{read::ZipFile, result::ZipError, ZipArchive};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Character {
    pub name: NonEmptyString,
    pub description: String,
    pub race: Race,
    pub class: Vec<CharacterClass>,
    pub cybernetics: Vec<Cybernetic>,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: NonEmptyString::new(String::from("Stanger")).unwrap(),
            description: Default::default(),
            race: Default::default(),
            class: Default::default(),
            cybernetics: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Race {
    pub name: NonEmptyString,
    pub info: String,
    pub age: String,
    pub size: Vec<String>,
    pub speed: usize,
    pub languages: Vec<String>,
    // sub race
}

impl Default for Race {
    fn default() -> Self {
        Self {
            name: NonEmptyString::new(String::from("Unknown Race")).unwrap(),
            speed: Default::default(),
            info: Default::default(),
            age: Default::default(),
            size: vec![],
            languages: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Cybernetic {
    pub name: NonEmptyString,
    pub cost: usize,
    pub part: String,
    pub tag: String,
}

impl Default for Cybernetic {
    fn default() -> Self {
        Self {
            name: NonEmptyString::new(String::from("Unidentified Cybernetic")).unwrap(),
            cost: Default::default(),
            part: Default::default(),
            tag: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct Book {
    pub errors: Vec<String>,
    pub class: Vec<Class>,
    pub balance: Vec<Balance>,
}

pub const DEFAULT_BOOK_BYTES: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/base_game.zip"));

impl Default for Book {
    fn default() -> Self {
        match ZipArchive::new(io::Cursor::new(DEFAULT_BOOK_BYTES)) {
            Ok(mut archive) => Book::from(&mut archive),
            Err(_) => Book::new_empty(),
        }
    }
}

pub fn write_default_book(path: &Path) -> Result<Book, ZipBookError> {
    File::create_new(path)?.write(DEFAULT_BOOK_BYTES)?;
    Ok(Book::default())
}

impl Book {
    pub fn new_empty() -> Self {
        Self {
            errors: vec![],
            class: vec![],
            balance: vec![],
        }
    }

    fn read_zip_file(&mut self, file: &mut ZipFile) {
        let meta = match FileMeta::from(file) {
            Ok(fm) => fm,
            Err(err) => {
                self.errors.push(err);
                return;
            }
        };

        match meta.format.as_str() {
            "lua" => {}
            _ => self.errors.push(format!("Not supported format.")),
        }

        match meta.extension.as_str() {
            "class" => match Class::from_lua_chunk(meta.name, meta.content.as_bytes()) {
                Ok(value) => self.class.push(value),
                Err(err) => self.errors.push(err.to_string()),
            },
            "balance" => match Balance::from_lua_chunk(meta.name, meta.content.as_bytes()) {
                Ok(value) => self.balance.push(value),
                Err(err) => self.errors.push(err.to_string()),
            },
            _ => self.errors.push(format!("Not supported type.")),
        }
    }
}

#[derive(Debug)]
pub enum ZipBookError {
    IO(io::Error),
    Zip(ZipError),
}

impl std::error::Error for ZipBookError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ZipBookError::IO(error) => error.source(),
            ZipBookError::Zip(zip_error) => zip_error.source(),
        }
    }
}

impl std::fmt::Display for ZipBookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ZipBookError::IO(error) => error.fmt(f),
            ZipBookError::Zip(zip_error) => zip_error.fmt(f),
        }
    }
}

impl From<io::Error> for ZipBookError {
    fn from(value: io::Error) -> Self {
        ZipBookError::IO(value)
    }
}

impl From<ZipError> for ZipBookError {
    fn from(value: ZipError) -> Self {
        ZipBookError::Zip(value)
    }
}

impl TryFrom<&Path> for Book {
    type Error = ZipBookError;
    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        Self::try_from(File::open(value)?)
    }
}

impl TryFrom<PathBuf> for Book {
    type Error = ZipBookError;
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Self::try_from(value.as_path())
    }
}

impl TryFrom<File> for Book {
    type Error = ZipBookError;
    fn try_from(value: File) -> Result<Self, Self::Error> {
        Ok(Self::from(&mut ZipArchive::new(value)?))
    }
}

impl<R: io::Read + io::Seek> From<&mut ZipArchive<R>> for Book {
    fn from(value: &mut ZipArchive<R>) -> Self {
        let mut book = Self::new_empty();
        for file_number in 0..value.len() {
            match value.by_index(file_number) {
                Ok(mut file) => {
                    if file.is_file() {
                        book.read_zip_file(&mut file)
                    }
                }
                Err(err) => book.errors.push(err.to_string()),
            };
        }
        book
    }
}

#[derive(Debug)]
struct FileMeta {
    format: NonEmptyString,
    extension: NonEmptyString,
    name: NonEmptyString,
    content: NonEmptyString,
}

impl FileMeta {
    fn from(file: &mut ZipFile) -> Result<Self, String> {
        let enclosed_name = file
            .enclosed_name()
            .ok_or_else(|| format!("File name not valid."))?;

        let format = match enclosed_name.extension() {
            Some(osstr) => NonEmptyString::new(osstr.to_string_lossy().to_string())
                .map_err(|_| format!("File extension missing."))?,
            None => return Err(format!("File extension missing.")),
        };

        let enclosed_stem = Path::new(
            enclosed_name
                .file_stem()
                .ok_or_else(|| format!("File stem missing."))?,
        );

        let name = match enclosed_stem.file_stem() {
            Some(osstr) => NonEmptyString::new(osstr.to_string_lossy().to_string())
                .map_err(|_| format!("File name missing."))?,
            None => return Err(format!("File name missing.")),
        };

        let extension = match enclosed_stem.extension() {
            Some(osstr) => NonEmptyString::new(osstr.to_string_lossy().to_string())
                .map_err(|_| format!("File extension missing."))?,
            None => return Err(format!("File extension missing.")),
        };

        let mut buf = String::new();
        let content = match io::Read::read_to_string(file, &mut buf) {
            Ok(_n) => match NonEmptyString::new(buf) {
                Ok(str) => str,
                Err(_) => return Err(format!("File is empty")),
            },
            Err(err) => return Err(err.to_string()),
        };

        Ok(Self {
            format,
            extension,
            name,
            content,
        })
    }
}
