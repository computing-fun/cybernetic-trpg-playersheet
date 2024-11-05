use std::{cell::RefCell, error::Error, fmt::Debug, path::PathBuf, rc::Rc};

use gtk4::{
    glib::ExitCode, prelude::*, Align, Application, ApplicationWindow, Box, Button, Entry,
    GestureClick, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow, Separator, SpinButton,
};
use non_empty_string::NonEmptyString;
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
pub enum MainMenuResult {
    CharacterEditor(Option<String>),
    RaceEditor(Option<String>),
    ClassEditor(Option<String>),
    CyberneticEditor(Option<String>),
}

pub fn main_menu(book: Rc<Book>) -> Option<MainMenuResult> {
    let result: Rc<RefCell<Option<MainMenuResult>>> = Rc::new(RefCell::new(None));
    let result_ref = Rc::clone(&result);
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title(book.name())
            .default_width(720)
            .default_height(480)
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

        type SelectAction = fn(String) -> MainMenuResult;

        fn character_select(name: String) -> MainMenuResult {
            MainMenuResult::CharacterEditor(Some(name))
        }

        fn race_select(name: String) -> MainMenuResult {
            MainMenuResult::RaceEditor(Some(name))
        }

        fn class_select(name: String) -> MainMenuResult {
            MainMenuResult::ClassEditor(Some(name))
        }

        fn cybernetic_select(name: String) -> MainMenuResult {
            MainMenuResult::CyberneticEditor(Some(name))
        }

        [
            (
                "Characters",
                book.table_of_contents::<Character>().unwrap_or_default(),
                MainMenuResult::CharacterEditor(None),
                character_select as SelectAction,
            ),
            (
                "Races",
                book.table_of_contents::<Race>().unwrap_or_default(),
                MainMenuResult::RaceEditor(None),
                race_select as SelectAction,
            ),
            (
                "Classes",
                book.table_of_contents::<Class>().unwrap_or_default(),
                MainMenuResult::ClassEditor(None),
                class_select as SelectAction,
            ),
            (
                "Cybernetics",
                book.table_of_contents::<Cybernetic>().unwrap_or_default(),
                MainMenuResult::CyberneticEditor(None),
                cybernetic_select as SelectAction,
            ),
        ]
        .into_iter()
        .for_each(|(name, list, create_result, select_action)| {
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

            let result_ref_clicked = Rc::clone(&result_ref);
            let window_ref = window.clone();
            header_new_btn.connect_clicked(move |_btn| {
                *result_ref_clicked.borrow_mut() = Some(create_result.clone());
                window_ref.close();
            });

            header.append(&header_new_btn);

            content.append(&header);

            let content_window_ref = content_window.clone();
            sidebar_btn.connect_clicked(move |_btn| {
                content_window_ref
                    .vadjustment()
                    .set_value(header.allocation().y() as f64);
            });

            for item in list {
                let btn = Button::new();
                btn.set_label(&item);
                btn.set_has_frame(false);
                let result_ref_clicked = Rc::clone(&result_ref);
                let window_ref = window.clone();
                btn.connect_clicked(move |_btn| {
                    *result_ref_clicked.borrow_mut() = Some(select_action(item.to_string()));
                    window_ref.close();
                });
                content.append(&btn);
            }

            let separator = Separator::new(Orientation::Horizontal);
            content.append(&separator);
        });

        window.show();
    });
    match app.run() == ExitCode::SUCCESS {
        true => result.take(),
        false => None,
    }
}

pub fn cybernetics_editor(book: Rc<Book>, name: Option<String>) -> ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Cybernetics Lab")
            .default_width(720)
            .default_height(480)
            .build();
        let window_child = Box::new(Orientation::Vertical, 10);
        window_child.set_margin_top(10);
        window_child.set_margin_bottom(10);
        window_child.set_margin_start(10);
        window_child.set_margin_end(10);
        window_child.set_halign(Align::Center);
        window.set_child(Some(&window_child));

        let window_ref = window.clone();
        let cybernetic = match &name {
            Some(name) => match book.read::<Cybernetic>(name) {
                Ok(cybernetic) => cybernetic,
                Err(err) => {
                    window_ref.show();
                    window_ref.close();
                    error(err);
                    return;
                }
            },
            None => Cybernetic::default(),
        };

        let name = Box::new(Orientation::Horizontal, 10);
        window_child.append(&name);

        let name_label = Label::new(Some("Name"));
        name.append(&name_label);

        let name_input = Entry::new();
        name_input.set_text(cybernetic.name.as_str());
        name.append(&name_input);

        let cost = Box::new(Orientation::Horizontal, 10);
        window_child.append(&cost);

        let cost_label = Label::new(Some("Point cost"));
        cost.append(&cost_label);

        let cost_input = SpinButton::with_range(0.0, 10.0, 1.0);
        cost_input.set_value(cybernetic.cost as f64);
        cost.append(&cost_input);

        let part = Box::new(Orientation::Horizontal, 10);
        window_child.append(&part);

        let part_label = Label::new(Some("Body Part"));
        part.append(&part_label);

        let part_input = Entry::new();
        part_input.set_text(&cybernetic.part);
        part.append(&part_input);

        let tag = Box::new(Orientation::Horizontal, 10);
        window_child.append(&tag);

        let tag_label = Label::new(Some("Tag"));
        tag.append(&tag_label);

        let tag_input = Entry::new();
        tag_input.set_text(&cybernetic.tag);
        tag.append(&tag_input);

        let save_btn = Button::new();
        save_btn.set_label("Save and Exit");
        let book_holder = Rc::clone(&book);
        let window_ref = window.clone();
        save_btn.connect_clicked(move |_btn| {
            let updated = Cybernetic {
                name: NonEmptyString::new(name_input.text().to_string()).unwrap(),
                cost: cost_input.value() as usize,
                part: part_input.text().to_string(),
                tag: tag_input.text().to_string(),
            };
            match book_holder.write(&updated) {
                Ok(_) => {
                    window_ref.close();
                }
                Err(err) => {
                    error(err);
                }
            };
        });
        window_child.append(&save_btn);

        window.show();
    });
    app.run()
}
