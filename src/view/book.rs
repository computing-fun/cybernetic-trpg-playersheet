use std::{cell::RefCell, error::Error, rc::Rc};

use gtk4::{
    glib::ExitCode, prelude::*, Application, ApplicationWindow, Box, Button, Label, ListBox,
    ListBoxRow, Orientation, ScrolledWindow, Separator,
};

use crate::{
    book::{Book, Sheet},
    sheet::{Character, Class, Cybernetic, Race},
};

use super::{ViewSwitcher, APP_ID};

struct Helper<'a> {
    book: &'a Book,
    content_window: &'a ScrolledWindow,
    content: &'a Box,
    sidebar: &'a ListBox,
}

impl Helper<'_> {
    fn add_error<E>(&self, err: E)
    where
        E: Error,
    {
        self.content.append(&Label::new(Some(&err.to_string())));
    }

    fn add_anchor(&self, name: &str, target: impl WidgetExt) {
        let sidebar_btn = Button::new();
        sidebar_btn.set_label(name);
        sidebar_btn.set_has_frame(false);

        let content_window_ref = self.content_window.clone();
        sidebar_btn.connect_clicked(move |_btn| {
            content_window_ref
                .vadjustment()
                .set_value(target.allocation().y() as f64);
        });

        let sibebar_row = ListBoxRow::new();
        sibebar_row.set_child(Some(&sidebar_btn));
        self.sidebar.append(&sibebar_row);
    }

    fn add_header_with_anchor<CREATOR>(&self, name: &str, on_create: CREATOR)
    where
        CREATOR: Fn(&Button) + 'static,
    {
        let top_separator = Separator::new(Orientation::Horizontal);
        self.content.append(&top_separator);

        let header = Box::new(Orientation::Horizontal, 10);

        let label = Label::new(Some(name));
        label.set_hexpand(true);
        header.append(&label);

        let btn_separator = Separator::new(Orientation::Vertical);
        header.append(&btn_separator);

        let new_btn = Button::new();
        new_btn.set_label("Create");
        new_btn.set_has_frame(false);
        new_btn.connect_clicked(on_create);
        header.append(&new_btn);

        self.content.append(&header);
        self.add_anchor(name, header);
    }

    fn add_sheet_section<S, CREATOR, DISPLAY>(
        &self,
        name: &str,
        on_create: CREATOR,
        on_display: DISPLAY,
    ) where
        S: Sheet,
        CREATOR: Fn(&Button) + 'static,
        DISPLAY: Fn(&Self, S) + 'static,
    {
        self.add_header_with_anchor(name, on_create);
        match self.book.section::<S>() {
            Ok(list) => {
                list.into_iter().for_each(|c| match c {
                    Ok(sheet) => {
                        on_display(&self, sheet);
                    }
                    Err(err) => {
                        self.add_error(err);
                    }
                });
            }
            Err(err) => {
                self.add_error(err);
            }
        }
    }
}

pub fn full(book: Rc<Book>) -> Option<ViewSwitcher> {
    let result: Rc<RefCell<Option<ViewSwitcher>>> = Rc::new(RefCell::new(None));
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

        let helper = Helper {
            book: &book,
            content_window: &content_window,
            content: &content,
            sidebar: &sidebar,
        };

        helper.add_sheet_section(
            "Characters",
            |_btn| {},
            |helper, sheet: Character| {
                helper
                    .content
                    .append(&Label::new(Some(&serde_json::json!(&sheet).to_string())));
            },
        );

        helper.add_sheet_section(
            "Races",
            |_btn| {},
            |helper, sheet: Race| {
                helper
                    .content
                    .append(&Label::new(Some(&serde_json::json!(&sheet).to_string())));
            },
        );

        helper.add_sheet_section(
            "Classes",
            |_btn| {},
            |helper, sheet: Class| {
                helper
                    .content
                    .append(&Label::new(Some(&serde_json::json!(&sheet).to_string())));
            },
        );

        helper.add_sheet_section(
            "Cybernetics",
            |_btn| {},
            |helper, sheet: Cybernetic| {
                helper
                    .content
                    .append(&Label::new(Some(&serde_json::json!(&sheet).to_string())));
            },
        );

        window.show();
    });
    match app.run() == ExitCode::SUCCESS {
        true => result.take(),
        false => None,
    }
}
