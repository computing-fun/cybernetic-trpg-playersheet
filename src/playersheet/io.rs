use std::{fs, path::PathBuf};

use super::PlayerSheet;

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
    fn not_selected() -> Self {
        Error::IO(std::io::Error::new(
            std::io::ErrorKind::Interrupted,
            "No file was selected",
        ))
    }
}

pub const EXTENSION: &str = "playersheet";
pub const EXTENSIONS: &[&str; 1] = &[EXTENSION];

pub fn read(path: &mut PathBuf) -> Result<PlayerSheet, Error> {
    path.set_extension(EXTENSION);
    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(err) => return Err(Error::IO(err)),
    };

    match serde_json::from_reader(file) {
        Ok(sheet) => Ok(sheet),
        Err(err) => Err(Error::Parse(err)),
    }
}

pub fn write(path: &mut PathBuf, sheet: &PlayerSheet) -> Result<(), Error> {
    path.set_extension(EXTENSION);
    let file = match fs::File::create(path) {
        Ok(file) => file,
        Err(err) => return Err(Error::IO(err)),
    };

    match serde_json::to_writer(file, sheet) {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::Parse(err)),
    }
}

pub fn read_arg() -> Option<(PlayerSheet, PathBuf)> {
    let mut path = std::env::args_os().nth(1).map(PathBuf::from)?;
    let sheet = read(&mut path).ok()?;
    Some((sheet, path))
}

pub fn read_dialog() -> Result<(PlayerSheet, PathBuf), Error> {
    let mut path = rfd::FileDialog::new()
        .set_title("Which player sheet do you want to pick up and read?")
        .add_filter(EXTENSION, EXTENSIONS)
        .set_file_name(PlayerSheet::default().name)
        .pick_file()
        .ok_or(Error::not_selected())?;
    let sheet = read(&mut path)?;
    Ok((sheet, path))
}

pub fn write_dialog(sheet: &PlayerSheet) -> Result<PathBuf, Error> {
    let mut path = rfd::FileDialog::new()
        .set_title("Where should we write and store this player sheet?")
        .add_filter(EXTENSION, EXTENSIONS)
        .set_file_name(&sheet.name)
        .save_file()
        .ok_or(Error::not_selected())?;
    write(&mut path, sheet)?;
    Ok(path)
}

pub fn create_or_open_dialog() -> Result<(PlayerSheet, PathBuf), Error> {
    let run_result = match super::create_or_open_dialog_app::run() {
        Ok(run_result) => run_result,
        Err(_err) => return Err(Error::not_selected()),
    };

    let message = match run_result {
        Some(message) => message,
        None => return Err(Error::not_selected()),
    };

    match message {
        super::create_or_open_dialog_app::Message::None => Err(Error::not_selected()),
        super::create_or_open_dialog_app::Message::Create => {
            let sheet = PlayerSheet::default();
            let path = write_dialog(&sheet)?;
            Ok((sheet, path))
        }
        super::create_or_open_dialog_app::Message::Open => read_dialog(),
    }
}

pub fn read_arg_or_create_or_open_dialog() -> Result<(PlayerSheet, PathBuf), Error> {
    if let Some(values) = read_arg() {
        return Ok(values);
    }
    create_or_open_dialog()
}
