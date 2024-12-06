pub mod ability;
pub mod class;
mod lua_object;

use std::{
    error::Error,
    fs::File,
    io::{self, Read},
    path::Path,
};

use class::{Balance, CharacterClass, Class};
use lua_object::FromLua;
use non_empty_string::NonEmptyString;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use tar::{Archive, Entries, Entry};

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

#[derive(Debug, Default)]
pub struct Book {
    pub errors: Vec<String>,
    pub class: Vec<Class>,
    pub balance: Vec<Balance>,
}

impl Book {
    pub fn read<P>(path: P) -> Result<Self, Box<dyn std::error::Error>>
    where
        P: AsRef<Path>,
    {
        let mut book = Book::default();
        let file = File::open(path)?;
        let mut archive = Archive::new(file);
        let entries = archive.entries()?;
        entries.for_each(|entry_result| book.read_entry_result(entry_result));
        Ok(book)
    }

    fn read_entry_result(&mut self, entry_result: Result<Entry<File>, io::Error>) {
        match entry_result {
            Ok(mut entry) => self.read_entry(&mut entry),
            Err(err) => self.errors.push(err.to_string()),
        }
    }

    fn read_entry(&mut self, entry: &mut Entry<File>) {
        let name_ext = match read_name_ext(&entry) {
            Ok(name_ext) => name_ext,
            Err(err) => {
                self.errors.push(err);
                return;
            }
        };

        match name_ext.extension.as_str() {
            "class" => self.read_class(entry, name_ext.name),
            "balance" => self.read_balance(entry, name_ext.name),
            _ => self.errors.push(String::from("Could not read ext.")),
        }
    }

    fn read_class(&mut self, entry: &mut Entry<File>, name: NonEmptyString) {
        match read_chunk_lua(entry, name) {
            Ok(value) => self.class.push(value),
            Err(err) => self.errors.push(err),
        }
    }

    fn read_balance(&mut self, entry: &mut Entry<File>, name: NonEmptyString) {
        match read_chunk_lua(entry, name) {
            Ok(value) => self.balance.push(value),
            Err(err) => self.errors.push(err),
        }
    }
}

struct NameExt {
    name: NonEmptyString,
    extension: NonEmptyString,
}

fn read_name_ext(entry: &Entry<File>) -> Result<NameExt, String> {
    let full_path = match entry.path() {
        Ok(path) => path,
        Err(err) => return Err(err.to_string()),
    };

    let file_stem_opt = match full_path.extension() {
        Some(lua_ext) if lua_ext == "lua" => full_path.file_stem(),
        _ => return Err(String::from("The lua file extension is missing.")),
    };

    let file_stem_osstr = match file_stem_opt {
        Some(osstr) => osstr,
        None => return Err(String::from("The file name is missing.")),
    };

    let path = Path::new(file_stem_osstr);

    let name = match path.file_stem() {
        Some(osstr) => match NonEmptyString::new(osstr.to_string_lossy().to_string()) {
            Ok(non_empty) => non_empty,
            Err(_) => return Err(String::from("The file name is missing.")),
        },
        None => return Err(String::from("The file name is missing.")),
    };

    let extension = match path.extension() {
        Some(osstr) => match NonEmptyString::new(osstr.to_string_lossy().to_string()) {
            Ok(non_empty) => non_empty,
            Err(_) => return Err(String::from("The file extension is missing.")),
        },
        None => return Err(String::from("The file extension is missing.")),
    };

    Ok(NameExt { name, extension })
}

fn read_chunk(entry: &mut Entry<File>) -> Result<NonEmptyString, String> {
    let mut buf = String::new();
    if let Err(err) = entry.read_to_string(&mut buf) {
        return Err(err.to_string());
    }
    match NonEmptyString::new(buf) {
        Ok(non_empty) => Ok(non_empty),
        Err(_) => Err(String::from("No date was read.")),
    }
}

fn read_chunk_lua<T>(entry: &mut Entry<File>, name: NonEmptyString) -> Result<T, String>
where
    T: FromLua,
{
    T::from_chunk(name, read_chunk(entry)?.as_str()).map_err(|err| err.to_string())
}
