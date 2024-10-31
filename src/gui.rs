use std::{cell::RefCell, rc::Rc};

use glib::ExitCode;
use gtk4::{prelude::*, Application, ApplicationWindow, Box, Button, Orientation};
use rfd::FileDialog;

use crate::{
    archive,
    sheet::{Catalog, Character, Class, Race},
    APP_ID,
};

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

#[derive(Debug)]
pub enum CreatorOptions {
    Character,
    Race,
    Class,
}

pub fn creator() {
    let result: Rc<RefCell<Option<CreatorOptions>>> = Rc::new(RefCell::new(None));
    let result_app = Rc::clone(&result);
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Creator")
            .default_width(250)
            .default_height(250)
            .build();

        let area = Box::new(Orientation::Vertical, 10);
        area.set_margin_start(10);
        area.set_margin_end(10);
        area.set_margin_top(10);
        area.set_margin_bottom(10);

        let char_btn = Button::with_label("Create new character");
        let race_btn = Button::with_label("Create new race");
        let class_btn = Button::with_label("Create new class");

        let value = result_app.clone();
        let window_weak = window.downgrade();
        char_btn.connect_clicked(move |_| {
            *value.borrow_mut() = Some(CreatorOptions::Character);
            window_weak.upgrade().unwrap().close();
        });

        let value = result_app.clone();
        let window_weak = window.downgrade();
        race_btn.connect_clicked(move |_| {
            *value.borrow_mut() = Some(CreatorOptions::Race);
            window_weak.upgrade().unwrap().close();
        });

        let value = result_app.clone();
        let window_weak = window.downgrade();
        class_btn.connect_clicked(move |_| {
            *value.borrow_mut() = Some(CreatorOptions::Class);
            window_weak.upgrade().unwrap().close();
        });

        area.append(&char_btn);
        area.append(&race_btn);
        area.append(&class_btn);

        window.set_child(Some(&area));
        window.show();
    });
    if app.run() != ExitCode::SUCCESS {
        return;
    }

    match result.take() {
        Some(opt) => match opt {
            CreatorOptions::Character => file_saver(&Character::default()).ok(),
            CreatorOptions::Race => file_saver(&Race::default()).ok(),
            CreatorOptions::Class => file_saver(&Class::default()).ok(),
        },
        None => todo!(),
    };
}
