use std::path::PathBuf;

use rfd::{FileDialog, MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};

#[allow(dead_code)]
pub fn error<E>(err: E)
where
    E: std::error::Error,
{
    eprintln!("{}", err);
    MessageDialog::new()
        .set_level(MessageLevel::Error)
        .set_title("Error")
        .set_description(format!("{}", err))
        .show();
}

#[allow(dead_code)]
pub fn debug<D>(debug: D)
where
    D: std::fmt::Debug,
{
    eprintln!("{:?}", debug);
    MessageDialog::new()
        .set_level(MessageLevel::Warning)
        .set_title("Debug")
        .set_description(format!("{:?}", debug))
        .show();
}

const BOOK_EXTENSION: &str = "book.zip";
const BOOK_FILTER_NAME: &str = "Book Zip file";
const BOOK_FILTER_EXTENSIONS: &[&str; 1] = &[BOOK_EXTENSION];

pub fn book_file() -> FileDialog {
    FileDialog::new().add_filter(BOOK_FILTER_NAME, BOOK_FILTER_EXTENSIONS)
}

pub fn open_book() -> Option<PathBuf> {
    book_file()
        .set_title("Which book do you want to pick up?")
        .pick_file()
}

pub fn create_book() -> Option<PathBuf> {
    book_file()
        .set_title("Where should we create this new book?")
        .set_can_create_directories(true)
        .save_file()
        .map(|mut p| {
            p.set_extension(BOOK_EXTENSION);
            p
        })
}

pub enum OpenOrCreateBook {
    Open(PathBuf),
    Create(PathBuf),
}

const OPEN_BTN: &str = "Open";
const CREATE_BTN: &str = "Create";
const CANCEL_BTN: &str = "Cancel";

pub fn open_or_create_book() -> Option<OpenOrCreateBook> {
    if let MessageDialogResult::Custom(response) = MessageDialog::new()
        .set_level(rfd::MessageLevel::Info)
        .set_title("Create or Open book?")
        .set_description("Are we creating a new book or opening an existing one?")
        .set_buttons(MessageButtons::YesNoCancelCustom(
            CREATE_BTN.to_string(),
            OPEN_BTN.to_string(),
            CANCEL_BTN.to_string(),
        ))
        .show()
    {
        if response == OPEN_BTN {
            return Some(OpenOrCreateBook::Open(open_book()?));
        }
        if response == CREATE_BTN {
            return Some(OpenOrCreateBook::Create(create_book()?));
        }
    };
    return None;
}
