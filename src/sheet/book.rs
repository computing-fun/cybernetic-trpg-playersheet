use std::{
    fs::File,
    io::{self, Write},
    path::{Path, PathBuf},
};

use zip::{read::ZipFile, result::ZipError, ZipArchive};

use super::source::{Balance, Class, Cybernetic, Loader};

#[derive(Debug)]
pub struct Book {
    pub errors: Vec<String>,
    pub class: Vec<Class>,
    pub balance: Vec<Balance>,
    pub cybernetics: Vec<Cybernetic>,
}

impl Book {
    pub fn new_empty() -> Self {
        Self {
            errors: vec![],
            class: vec![],
            balance: vec![],
            cybernetics: vec![],
        }
    }
}

/// --- Default

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

pub fn write_default_book(path: &Path) -> Result<Book, BookMakerError> {
    File::create_new(path)?.write(DEFAULT_BOOK_BYTES)?;
    Ok(Book::default())
}

/// --- File Reader

enum Format {
    Lua,
}

impl TryFrom<&str> for Format {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "lua" => Ok(Format::Lua),
            _ => Err(()),
        }
    }
}

enum ContentType {
    Class,
    Balance,
}

impl TryFrom<&str> for ContentType {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "class" => Ok(ContentType::Class),
            "balance" => Ok(ContentType::Balance),
            _ => Err(()),
        }
    }
}

fn read_content(
    format: Format,
    content_type: ContentType,
    book: &mut Book,
    name: String,
    content: String,
) {
    match (format, content_type) {
        (Format::Lua, ContentType::Class) => {
            if let Some(value) = read_lua(book, name, content) {
                book.class.push(value);
            }
        }
        (Format::Lua, ContentType::Balance) => {
            if let Some(value) = read_lua(book, name, content) {
                book.balance.push(value);
            }
        }
    }
}

fn read_lua<T>(book: &mut Book, name: String, content: String) -> Option<T>
where
    T: Loader,
{
    match T::load(name, content) {
        Ok(tryfrom) => match tryfrom {
            Ok(value) => Some(value),
            Err(err) => {
                book.errors.push(err.to_string());
                return None;
            }
        },
        Err(err) => {
            book.errors.push(err.to_string());
            return None;
        }
    }
}

/// --- Book Makers --- From / TryFrom

impl TryFrom<&Path> for Book {
    type Error = BookMakerError;
    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        Self::try_from(File::open(value)?)
    }
}

impl TryFrom<PathBuf> for Book {
    type Error = BookMakerError;
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Self::try_from(value.as_path())
    }
}

impl TryFrom<File> for Book {
    type Error = BookMakerError;
    fn try_from(value: File) -> Result<Self, Self::Error> {
        Ok(Self::from(&mut ZipArchive::new(value)?))
    }
}

impl<R: io::Read + io::Seek> From<&mut ZipArchive<R>> for Book {
    fn from(value: &mut ZipArchive<R>) -> Self {
        let mut book = Self::new_empty();
        for file_number in 0..value.len() {
            match value.by_index(file_number) {
                Ok(mut file) => read_file(&mut book, &mut file),
                Err(err) => book.errors.push(err.to_string()),
            };
        }
        book
    }
}

fn read_file(book: &mut Book, mut file: &mut ZipFile) {
    if !file.is_file() {
        return;
    }

    let enclosed_name = match file.enclosed_name() {
        Some(path) => path,
        None => {
            book.errors.push(format!("Could not find enclosed name."));
            return;
        }
    };

    let file_name = match enclosed_name.file_stem() {
        Some(osstr) => Path::new(osstr),
        None => {
            book.errors.push(format!("Could not find file stem."));
            return;
        }
    };

    let format = match enclosed_name.extension() {
        Some(osstr) => match Format::try_from(osstr.to_string_lossy().as_ref()) {
            Ok(format) => format,
            Err(_) => {
                book.errors.push(format!("Not supported format."));
                return;
            }
        },
        None => {
            book.errors.push(format!("Could not find file format."));
            return;
        }
    };

    let content_type = match file_name.extension() {
        Some(osstr) => match ContentType::try_from(osstr.to_string_lossy().as_ref()) {
            Ok(extension) => extension,
            Err(_) => {
                book.errors.push(format!("Not supported extension."));
                return;
            }
        },
        None => {
            book.errors.push(format!("Could not find file extension."));
            return;
        }
    };

    let name = match file_name.file_stem() {
        Some(osstr) => osstr.to_string_lossy().to_string(),
        None => {
            book.errors.push(format!("Could not find file name."));
            return;
        }
    };

    let mut content = String::new();
    match io::Read::read_to_string(&mut file, &mut content) {
        Ok(num) => {
            if num == 0 || content.len() == 0 {
                book.errors.push(format!("Contents of file is empty."));
                return;
            }
        }
        Err(err) => {
            book.errors.push(err.to_string());
            return;
        }
    };

    read_content(format, content_type, book, name, content);
}

/// --- Book Maker Error

#[derive(Debug)]
pub enum BookMakerError {
    IO(io::Error),
    Zip(ZipError),
}

impl std::error::Error for BookMakerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BookMakerError::IO(error) => error.source(),
            BookMakerError::Zip(zip_error) => zip_error.source(),
        }
    }
}

impl std::fmt::Display for BookMakerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BookMakerError::IO(error) => error.fmt(f),
            BookMakerError::Zip(zip_error) => zip_error.fmt(f),
        }
    }
}

impl From<io::Error> for BookMakerError {
    fn from(value: io::Error) -> Self {
        BookMakerError::IO(value)
    }
}

impl From<ZipError> for BookMakerError {
    fn from(value: ZipError) -> Self {
        BookMakerError::Zip(value)
    }
}
