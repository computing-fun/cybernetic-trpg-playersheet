/// Trait for types that can be archived, associating each type with a unique file extension.
pub trait Archivable: serde::Serialize + serde::de::DeserializeOwned {
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
