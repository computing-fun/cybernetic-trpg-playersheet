use std::{fs::File, io, path::Path};

use zip::ZipArchive;

use super::book::{Book, Src};

pub const DEFAULT_BOOK_BYTES: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/base_game.zip"));

impl Default for Book {
    fn default() -> Self {
        match ZipArchive::new(io::Cursor::new(DEFAULT_BOOK_BYTES)) {
            Ok(mut archive) => Book::from(&mut archive),
            Err(_) => Book::new(),
        }
    }
}

pub fn write_default_book(path: &Path) -> Result<Book, BookMakerError> {
    File::create_new(path)?.write(DEFAULT_BOOK_BYTES)?;
    Ok(Book::default())
}

/// --- File Reader

enum ContentType {
    Race,
    Class,
    Balance,
    Cybernetic,
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
        let mut book = Self::new();
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
    let mut content = String::new();
    match io::Read::read_to_string(&mut file, &mut content) {
        Ok(num) => {
            if num == 0 || content.len() == 0 {
                return;
            }
        }
        Err(err) => {
            return;
        }
    };

    read_content(format, content_type, book, name, content);
}

impl Book<'_> {
    fn write_zip_file(&mut self, file: zip::read::ZipFile) {
        if !file.is_file() {
            return;
        }

        let file_name = {
            let file_name = match file.enclosed_name() {
                Some(file_name) => file_name,
                None => return,
            };

            if file_name.extension().and_then(|osstr| osstr.to_str()) != Some("lua") {
                return;
            }

            match file_name.file_stem() {
                Some(file_name) => Path::new(file_name),
                None => return,
            }
        };

        let content_type = file_name
            .extension()
            .and_then(|osstr| ContentType::try_from(osstr.to_string_lossy().as_ref()).ok())
            .unwrap_or_else(|| return);

        let name = file_name
            .file_stem()
            .map(|osstr| osstr.to_string_lossy().to_string().into_boxed_str())
            .unwrap_or_else(|| return);

        let mut content = String::new();
        if let Err(_) = io::Read::read_to_string(&mut file, &mut content) {
            return;
        }

        if content.is_empty() {
            return;
        }

        let src = Src::load(content).unwrap_or_else(|_| return);

        match content_type {
            ContentType::Race => self.race.write(name, src),
            ContentType::Class => self.class.write(name, src),
            ContentType::Balance => self.balance.write(name, src),
            ContentType::Cybernetic => self.cybernetics.write(name, src),
        }
    }
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
