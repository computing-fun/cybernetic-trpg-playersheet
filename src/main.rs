use std::{cell::RefCell, rc::Rc};

use gtk4::{glib::ExitCode, prelude::*, Application, ApplicationWindow, Box, Button, Orientation};
use rfd::FileDialog;
use sheet::Catalog;

mod archive;
mod sheet;

pub const APP_ID: &str = "org.computingfun.cybernetic-trpg";

fn main() -> ExitCode {
    let r = main_menu();
    println!("{:?}", r);
    ExitCode::SUCCESS
}

/// Fetches the first command-line argument and attempts to return a `Catalog` entry.
/// Returns [`None`] if the argument or matching catalog entry is not found.
pub fn arg() -> Option<Catalog> {
    std::env::args_os().nth(1).and_then(Catalog::lookup)
}

#[derive(Debug)]
pub enum MainMenuOptions {
    Open,
    Character,
    Race,
    Class,
}

pub fn main_menu() -> Option<MainMenuOptions> {
    let result: Rc<RefCell<Option<MainMenuOptions>>> = Rc::new(RefCell::new(None));
    let result_ref = Rc::clone(&result);
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Creator")
            .default_width(250)
            .default_height(250)
            .resizable(false)
            .build();

        let area = Box::new(Orientation::Vertical, 10);
        area.set_margin_start(10);
        area.set_margin_end(10);
        area.set_margin_top(10);
        area.set_margin_bottom(10);

        let open_btn = Button::with_label("Open sheet");
        let char_btn = Button::with_label("Create new character");
        let race_btn = Button::with_label("Create new race");
        let class_btn = Button::with_label("Create new class");

        let value = result_ref.clone();
        let window_ref = window.downgrade();
        open_btn.connect_clicked(move |_| {
            *value.borrow_mut() = Some(MainMenuOptions::Open);
            if let Some(w) = window_ref.upgrade() {
                w.close();
            }
        });

        let value = result_ref.clone();
        let window_ref = window.downgrade();
        char_btn.connect_clicked(move |_| {
            *value.borrow_mut() = Some(MainMenuOptions::Character);
            if let Some(w) = window_ref.upgrade() {
                w.close();
            }
        });

        let value = result_ref.clone();
        let window_ref = window.downgrade();
        race_btn.connect_clicked(move |_| {
            *value.borrow_mut() = Some(MainMenuOptions::Race);
            if let Some(w) = window_ref.upgrade() {
                w.close();
            }
        });

        let value = result_ref.clone();
        let window_ref = window.downgrade();
        class_btn.connect_clicked(move |_| {
            *value.borrow_mut() = Some(MainMenuOptions::Class);
            if let Some(w) = window_ref.upgrade() {
                w.close();
            }
        });

        area.append(&open_btn);
        area.append(&char_btn);
        area.append(&race_btn);
        area.append(&class_btn);

        window.set_child(Some(&area));
        window.show();
    });
    match app.run() == ExitCode::SUCCESS {
        true => result.take(),
        false => None,
    }
}

/// Opens a file dialog to allow the user to pick a file, then attempts to load
/// the corresponding `Catalog` entry based on file extension.
/// Returns [`None`] if no file is chosen or if no matching catalog entry is found.
pub fn file_picker() -> Option<Catalog> {
    FileDialog::new().pick_file().and_then(Catalog::lookup)
}

/// Prompts the user to select a location to save the provided `Archivable` instance.
/// Returns an error if the user cancels the save dialog or if the file extension is invalid.
pub fn file_saver<A>(archivable: &A) -> Result<(), archive::Error>
where
    A: archive::Archivable,
{
    let path = match FileDialog::new()
        .add_filter(A::EXTENSION, &[A::EXTENSION])
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
