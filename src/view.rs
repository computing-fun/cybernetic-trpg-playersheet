use std::{cell::RefCell, error::Error, fmt::Debug, rc::Rc};

use gtk4::{glib::ExitCode, prelude::*, Application, ApplicationWindow, Box, Button, Orientation};
use rfd::{FileDialog, MessageDialog, MessageLevel};

use crate::sheet;

pub const APP_ID: &str = "org.computingfun.cybernetic-trpg";

pub fn error<E>(err: E)
where
    E: Error,
{
    eprintln!("{}", err);
    MessageDialog::new()
        .set_level(MessageLevel::Error)
        .set_title("Cybernetic TRPG - Error")
        .set_description(format!("{}", err))
        .show();
}

pub fn debug<D>(debug: D)
where
    D: Debug,
{
    println!("{:?}", debug);
    MessageDialog::new()
        .set_level(MessageLevel::Warning)
        .set_title("Cybernetic TRPG - Debug")
        .set_description(format!("{:?}", debug))
        .show();
}

#[derive(Debug, Clone, Copy)]
pub enum MainMenuOptions {
    NewCharacter,
    NewRace,
    NewClass,
    Tester,
}

pub fn main_menu() -> Option<MainMenuOptions> {
    let result: Rc<RefCell<Option<MainMenuOptions>>> = Rc::new(RefCell::new(None));
    let result_ref = Rc::clone(&result);
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Cybernetic TRPG")
            .default_width(0)
            .default_height(0)
            .resizable(false)
            .build();

        let area = Box::new(Orientation::Vertical, 10);
        area.set_margin_start(10);
        area.set_margin_end(10);
        area.set_margin_top(10);
        area.set_margin_bottom(10);

        for (label, opt) in [
            ("Create or Open character", MainMenuOptions::NewCharacter),
            ("Create or Open race", MainMenuOptions::NewRace),
            ("Create or Open class", MainMenuOptions::NewClass),
            ("Tester", MainMenuOptions::Tester),
        ] {
            let btn = Button::with_label(label);
            let value = result_ref.clone();
            let window_ref = window.downgrade();
            btn.connect_clicked(move |_| {
                *value.borrow_mut() = Some(opt);
                if let Some(w) = window_ref.upgrade() {
                    w.close();
                }
            });
            area.append(&btn);
        }

        window.set_child(Some(&area));
        window.show();
    });
    match app.run() == ExitCode::SUCCESS {
        true => result.take(),
        false => None,
    }
}
