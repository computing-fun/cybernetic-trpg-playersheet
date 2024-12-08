use std::{cell::RefCell, rc::Rc};

use gtk4::{
    glib::ExitCode, prelude::*, Application, ApplicationWindow, Box, Button, ListBox, ListBoxRow,
    Orientation, ScrolledWindow, Separator,
};
use non_empty_string::NonEmptyString;

use crate::book::Book;

use super::{basic_lable, name_tag_content, APP_ID};

struct Helper<'a> {
    content_window: &'a ScrolledWindow,
    content: &'a Box,
    sidebar: &'a ListBox,
}

impl Helper<'_> {
    fn add_anchor(&self, name: &str) {
        let target = Box::new(Orientation::Horizontal, 0);
        self.content.append(&target);

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
        window_child.append(&sidebar_window);

        let content_window = ScrolledWindow::new();
        window_child.append(&content_window);

        let sidebar = ListBox::new();
        sidebar.set_selection_mode(gtk4::SelectionMode::None);
        sidebar.set_width_request(200);
        sidebar_window.set_child(Some(&sidebar));

        let content = Box::new(Orientation::Vertical, 10);
        content.set_hexpand(true);
        content.set_margin_start(10);
        content.set_margin_end(10);
        content.set_margin_top(10);
        content.set_margin_bottom(10);
        content_window.set_child(Some(&content));

        let helper = Helper {
            content_window: &content_window,
            content: &content,
            sidebar: &sidebar,
        };

        helper.add_anchor("Class");
        for class in &book.class {
            let fields_maker = |name, details: Vec<NonEmptyString>| {
                let boxed = Box::new(Orientation::Horizontal, 30);
                boxed.append(&basic_lable(name));
                boxed.append(&basic_lable(details.join(",  ").as_str()));
                boxed
            };

            let details = Box::new(Orientation::Vertical, 30);
            details.set_hexpand(true);

            details.append(&basic_lable(class.description().as_str()));
            details.append(&fields_maker(
                "Astralic Types".into(),
                class.astralic_types(),
            ));
            details.append(&fields_maker("Saving Throws".into(), class.saving_throws()));
            details.append(&fields_maker("Skills".into(), class.skills(0)));

            content.append(&name_tag_content(class.name().as_str(), "Class", &details));
            content.append(&Separator::new(Orientation::Vertical));
        }

        helper.add_anchor("Balance");
        for balance in &book.balance {
            let details = basic_lable(balance.description().as_str());
            details.set_hexpand(true);

            content.append(&name_tag_content(
                balance.name().as_str(),
                "Balance",
                &details,
            ));

            content.append(&Separator::new(Orientation::Vertical));
        }

        window.show();
    });
    let mock_args: [String; 0] = [];
    match app.run_with_args(&mock_args) == ExitCode::SUCCESS {
        true => result.take(),
        false => None,
    }
}
