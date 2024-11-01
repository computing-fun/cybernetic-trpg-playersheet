/// Trait for types that can be archived, associating each type with a unique file extension.
pub trait Archivable: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug {
    const EXTENSION: &'static str;
}

/// Struct for handling file operations for a specific `Archivable` type.
#[derive(Debug, Clone)]
pub struct Archive<A>
where
    A: Archivable,
{
    index: std::path::PathBuf,
    _marker: std::marker::PhantomData<A>,
}

impl<A> AsRef<std::path::Path> for Archive<A>
where
    A: Archivable,
{
    /// Allows `Archive` to be used where a [`std::path::Path`] reference is needed.
    fn as_ref(&self) -> &std::path::Path {
        &self.index
    }
}

impl<A> Archive<A>
where
    A: Archivable,
{
    /// Creates a new `Archive` instance with the correct file extension.
    /// Returns [`None`] if setting the extension fails.
    pub fn new<P>(path: P) -> Option<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let mut index = path.as_ref().to_path_buf();
        match index.set_extension(A::EXTENSION) {
            true => Some(Self {
                index,
                _marker: std::marker::PhantomData,
            }),
            false => None,
        }
    }

    /// Fetches and deserializes data from the file into an instance of type `A`.
    pub fn fetch(&self) -> Result<A, Error>
    where
        A: Archivable,
    {
        let file = std::fs::File::open(self).map_err(Error::IO)?;
        serde_json::from_reader(file).map_err(Error::Parse)
    }

    /// Serializes and writes an instance of type `A` to the file.
    pub fn record(&self, archivable: &A) -> Result<(), Error>
    where
        A: Archivable,
    {
        let file = std::fs::File::create(self).map_err(Error::IO)?;
        serde_json::to_writer(file, archivable).map_err(Error::Parse)
    }
}

/// Error type for handling I/O and serialization issues during file operations.
#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Parse(serde_json::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(error) => write!(f, "I/O error: {}", error),
            Error::Parse(error) => write!(f, "Parse error: {}", error),
        }
    }
}

/// Opens a file dialog to allow the user to pick a file, then attempts to load
/// the corresponding `Catalog` entry based on file extension.
/// Returns [`None`] if no file is chosen or if no matching catalog entry is found.
pub fn file_picker() -> Option<sheet::Catalog> {
    FileDialog::new()
        .set_title("Cybernetic TRPG - Open book or sheet")
        .pick_file()
        .and_then(sheet::Catalog::lookup)
}

/// Prompts the user to select a location to save the provided `Archivable` instance.
/// Returns an error if the user cancels the save dialog or if the file extension is invalid.
pub fn file_saver<A>(archivable: &A) -> Result<(), archive::Error>
where
    A: archive::Archivable,
{
    let path = match FileDialog::new()
        .add_filter(A::EXTENSION, &[A::EXTENSION])
        .set_title(format!("Cybernetic TRPG - Save {}", A::EXTENSION))
        .save_file()
    {
        Some(path) => path,
        None => {
            return Err(archive::Error::IO(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Save canceled",
            )))
        }
    };

    let arch = match archive::Archive::new(path) {
        Some(arch) => arch,
        None => {
            return Err(archive::Error::IO(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid file extension",
            )))
        }
    };

    arch.record(archivable)
}

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

impl Archivable for Character {
    const EXTENSION: &'static str = "character-sheet";
}

impl Archivable for Race {
    const EXTENSION: &'static str = "race-sheet";
}

impl Archivable for Class {
    const EXTENSION: &'static str = "class-sheet";
}
