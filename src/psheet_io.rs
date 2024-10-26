use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use rfd::FileDialog;

use crate::playersheet::{Class, PlayerSheet, Race};

pub enum Error {
    IO(std::io::Error),
    Parse(serde_json::Error),
}

impl Error {
    fn not_selected() -> Self {
        Error::IO(std::io::Error::new(
            std::io::ErrorKind::Interrupted,
            "No file was selected",
        ))
    }
}

pub const EXTENSION: &str = "playersheet";
pub const EXTENSIONS: &[&str; 1] = &[EXTENSION];

pub fn path_with_extension<P>(path: P) -> PathBuf
where
    P: AsRef<std::path::Path>,
{
    let mut path_ext = path.as_ref().to_path_buf();
    path_ext.set_extension(EXTENSION);
    return path_ext;
}

pub fn read<P>(path: P) -> Result<PlayerSheet, Error>
where
    P: AsRef<std::path::Path>,
{
    let mut file = match File::options()
        .read(true)
        .create(true)
        .write(true)
        .open(path_with_extension(path))
    {
        Ok(file) => file,
        Err(err) => return Err(Error::IO(err)),
    };

    let mut buf = String::new();
    let read = match file.read_to_string(&mut buf) {
        Ok(read) => read,
        Err(err) => return Err(Error::IO(err)),
    };

    if read == 0 || buf.is_empty() {
        return Ok(PlayerSheet {
            name: "Hero".to_string(),
            race: Race {
                name: "Unknown".to_string(),
            },
            class: Class {
                name: "Classless".to_string(),
            },
        });
    }

    match serde_json::from_str(&buf) {
        Ok(sheet) => Ok(sheet),
        Err(err) => return Err(Error::Parse(err)),
    }
}

pub fn write<P>(path: P, sheet: &PlayerSheet) -> Result<(), Error>
where
    P: AsRef<std::path::Path>,
{
    let json = match serde_json::to_string(sheet) {
        Ok(json) => json,
        Err(err) => return Err(Error::Parse(err)),
    };

    let mut file = match File::create(path_with_extension(path)) {
        Ok(file) => file,
        Err(err) => return Err(Error::IO(err)),
    };

    match file.write_all(json.as_bytes()) {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::IO(err)),
    }
}

pub fn create_or_open_dialog() {
    todo!()
}

pub fn read_dialog() -> Result<(PlayerSheet, PathBuf), Error> {
    let path = FileDialog::new()
        .set_title("Which player sheet do you want to pick up and read")
        .add_filter(EXTENSION, EXTENSIONS)
        .set_file_name("Hero")
        .pick_file()
        .ok_or(Error::not_selected())?;
    Ok((read(&path)?, path))
}

pub fn write_dialog(sheet: &PlayerSheet) -> Result<PathBuf, Error> {
    let path = FileDialog::new()
        .set_title("Where should we write and store this player sheet")
        .add_filter(EXTENSION, EXTENSIONS)
        .set_file_name("Hero")
        .save_file()
        .ok_or(Error::not_selected())?;
    write(&path, sheet)?;
    Ok(path)
}
