use std::{error::Error, fmt::Debug, path::PathBuf};

use gtk4::prelude::*;
use rfd::{FileDialog, MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};

pub const APP_ID: &str = "org.computingfun.cybernetic-trpg";

#[allow(dead_code)]
pub fn error<E>(err: E)
where
    E: Error,
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
    D: Debug,
{
    println!("{:?}", debug);
    MessageDialog::new()
        .set_level(MessageLevel::Warning)
        .set_title("Debug")
        .set_description(format!("{:?}", debug))
        .show();
}

pub fn open_book() -> Option<PathBuf> {
    FileDialog::new()
        .set_title("Which book do you want to pick up?")
        .add_filter("Book", &["book"])
        .set_can_create_directories(true)
        .pick_file()
}

pub fn create_book() -> Option<PathBuf> {
    FileDialog::new()
        .set_title("Where should we create this new book?")
        .add_filter("Book", &["book"])
        .set_can_create_directories(true)
        .save_file()
}

pub fn open_or_create_book() -> Option<PathBuf> {
    let open = String::from("Open");
    let create = String::from("Create");
    if let MessageDialogResult::Custom(response) = MessageDialog::new()
        .set_level(rfd::MessageLevel::Info)
        .set_title("Create or Open book?")
        .set_description("Are we creating a new book or opening an existing one?")
        .set_buttons(MessageButtons::YesNoCancelCustom(
            create.clone(),
            open.clone(),
            "Cancel".to_string(),
        ))
        .show()
    {
        if response == open {
            return open_book();
        }
        if response == create {
            return create_book();
        }
    };
    return None;
}
