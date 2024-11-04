use std::{cell::RefCell, error::Error, fmt::Debug, path::PathBuf, rc::Rc};

use gtk4::{
    glib::ExitCode, prelude::*, Application, ApplicationWindow, Box, Button, Entry, Label, ListBox,
    ListBoxRow, Orientation, ScrolledWindow, Separator, SpinButton,
};
use rfd::{FileDialog, MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};

use crate::{
    book::Book,
    sheet::{Character, Class, Cybernetic, Race},
};

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

#[allow(dead_code)]
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

#[derive(Debug, Clone)]
pub enum MainMenuOptions {
    CharacterEditor(Character),
    RaceEditor(Race),
    ClassEditor(Class),
    CyberneticEditor(Cybernetic),
}

pub fn main_menu(book: Rc<Book>) -> Option<MainMenuOptions> {
    let result: Rc<RefCell<Option<MainMenuOptions>>> = Rc::new(RefCell::new(None));
    let result_ref = Rc::clone(&result);
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title(book.name())
            .build();
        let window_child = Box::new(Orientation::Horizontal, 10);
        window.set_child(Some(&window_child));

        let sidebar_window = ScrolledWindow::new();
        sidebar_window.set_hscrollbar_policy(gtk4::PolicyType::Never);
        let sidebar = ListBox::new();
        sidebar_window.set_child(Some(&sidebar));
        sidebar.set_selection_mode(gtk4::SelectionMode::None);
        sidebar.set_width_request(200);
        window_child.append(&sidebar_window);

        let content_window = ScrolledWindow::new();
        content_window.set_hexpand(true);
        let content = Box::new(Orientation::Vertical, 10);
        content.set_margin_top(10);
        content.set_margin_end(10);
        content_window.set_child(Some(&content));
        window_child.append(&content_window);

        for (name, list) in [
            (
                "Characters",
                book.table_of_contents::<Character>().unwrap_or_default(),
            ),
            (
                "Races",
                book.table_of_contents::<Race>().unwrap_or_default(),
            ),
            (
                "Classes",
                book.table_of_contents::<Class>().unwrap_or_default(),
            ),
            (
                "Cybernetics",
                book.table_of_contents::<Cybernetic>().unwrap_or_default(),
            ),
        ] {
            let sidebar_btn = Button::new();
            sidebar_btn.set_label(name);
            sidebar_btn.set_has_frame(false);
            let sidebar_box = ListBoxRow::new();
            sidebar_box.set_child(Some(&sidebar_btn));
            sidebar.append(&sidebar_box);

            let header = Box::new(Orientation::Horizontal, 10);

            let header_label = Label::new(Some(name));
            header_label.set_hexpand(true);
            header.append(&header_label);

            let header_separator = Separator::new(Orientation::Vertical);
            header.append(&header_separator);

            let header_new_btn = Button::new();
            header_new_btn.set_label("Create");
            header_new_btn.set_has_frame(false);
            header.append(&header_new_btn);

            let header_new_name = Entry::new();
            header_new_name.set_placeholder_text(Some("Enter name here!"));
            header_new_name.set_visible(false);
            header.append(&header_new_name);

            header_new_btn.connect_clicked(move |_btn| {
                if !header_new_name.get_visible() {
                    header_new_name.set_visible(true);
                    header_new_name.grab_focus();
                    return;
                }
            });

            content.append(&header);

            let content_window_ref = content_window.clone();
            sidebar_btn.connect_clicked(move |_btn| {
                content_window_ref
                    .vadjustment()
                    .set_value(header.allocation().y() as f64);
            });

            for item in list {
                let label = Label::new(Some(&item));
                content.append(&label);
            }

            let separator = Separator::new(Orientation::Horizontal);
            content.append(&separator);
        }

        window.show();
    });
    match app.run() == ExitCode::SUCCESS {
        true => result.take(),
        false => None,
    }
}

pub fn cybernetics_editor(book: Rc<Book>, cybernetic_name: String) -> ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(move |app| {
        let cybernetic = book.read::<Cybernetic>(&cybernetic_name).unwrap();

        let window = ApplicationWindow::builder()
            .application(app)
            .title(cybernetic.name.to_string())
            .build();
        let window_child = Box::new(Orientation::Vertical, 10);
        window.set_child(Some(&window_child));

        let tag = Entry::new();
        tag.set_text(&cybernetic.tag);

        window_child.append(&tag);

        window.show();
    });
    app.run()
}
