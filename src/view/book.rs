use std::{cell::RefCell, error::Error, rc::Rc};

use gtk4::{
    glib::ExitCode, prelude::*, Application, ApplicationWindow, Box, Button, Label, ListBox,
    ListBoxRow, Orientation, ScrolledWindow, Separator,
};

use crate::book::Book;

use super::APP_ID;

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

    fn add_header_with_anchor<ADDED>(&self, name: &str, on_add: ADDED)
    where
        ADDED: Fn(&Button) + 'static,
    {
        let top_separator = Separator::new(Orientation::Horizontal);
        self.content.append(&top_separator);

        let header = Box::new(Orientation::Horizontal, 10);

        let label = Label::new(Some(name));
        label.set_hexpand(true);
        header.append(&label);

        let btn_separator = Separator::new(Orientation::Vertical);
        header.append(&btn_separator);

        let add_btn = Button::new();
        add_btn.set_label("Add");
        add_btn.set_has_frame(false);
        add_btn.connect_clicked(on_add);
        header.append(&add_btn);

        self.content.append(&header);
        self.add_anchor(name, header);
    }
}

pub fn full(book: Rc<Book>) -> Option<()> {
    let result: Rc<RefCell<Option<()>>> = Rc::new(RefCell::new(None));
    //let result_ref = Rc::clone(&result);
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(720)
            .default_height(480)
            .build();
        let window_child = Box::new(Orientation::Horizontal, 10);
        window.set_child(Some(&window_child));

        let sidebar_window = ScrolledWindow::new();
        sidebar_window.set_hscrollbar_policy(gtk4::PolicyType::Never);

        let sidebar = ListBox::new();
        sidebar.set_selection_mode(gtk4::SelectionMode::None);
        sidebar.set_width_request(200);

        sidebar_window.set_child(Some(&sidebar));
        window_child.append(&sidebar_window);

        let content_window = ScrolledWindow::new();

        let content = Box::new(Orientation::Vertical, 10);
        content.set_margin_start(10);
        content.set_margin_end(10);
        content.set_margin_top(10);
        content.set_margin_bottom(10);

        content_window.set_child(Some(&content));
        window_child.append(&content_window);

        let helper = Helper {
            book: &book,
            content_window: &content_window,
            content: &content,
            sidebar: &sidebar,
        };

        helper.add_header_with_anchor("Class", |_btn| {});
        for class in &book.class {
            let label = Label::new(Some(class.name().as_str()));
            label.set_selectable(true);
            content.append(&label);

            let details = Label::new(Some(&class.description()));
            details.set_wrap(true);
            details.set_selectable(true);
            content.append(&details);
        }

        helper.add_header_with_anchor("Balance", |_btn| {});
        for balance in &book.balance {
            let label = Label::new(Some(balance.name().as_str()));
            label.set_selectable(true);
            content.append(&label);

            let details = Label::new(Some(&balance.description()));
            details.set_wrap(true);
            details.set_selectable(true);
            content.append(&details);
        }

        window.show();
    });
    let mock_args: [String; 0] = [];
    match app.run_with_args(&mock_args) == ExitCode::SUCCESS {
        true => result.take(),
        false => None,
    }
}
