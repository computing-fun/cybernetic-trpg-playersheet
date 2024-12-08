use gtk4::{prelude::*, Align, Box as GBox, Label, Orientation, Separator, Widget};

pub mod book;

pub const APP_ID: &str = "org.computingfun.cybernetic-trpg";

fn basic_lable(text: &str) -> Label {
    let label = Label::new(Some(text));
    label.set_valign(Align::Start);
    label.set_selectable(true);
    label.set_wrap(true);
    label.set_width_request(100);
    label
}

fn name_tag(name: &str, tag: &str) -> GBox {
    let g_box = GBox::new(Orientation::Vertical, 10);
    g_box.append(&basic_lable(name));
    g_box.append(&Separator::new(Orientation::Vertical));
    g_box.append(&basic_lable(tag));
    g_box
}

fn name_tag_content(name: &str, tag: &str, content: &impl IsA<Widget>) -> GBox {
    let g_box = GBox::new(Orientation::Horizontal, 30);
    g_box.append(&name_tag(name, tag));
    g_box.append(content);
    g_box
}
