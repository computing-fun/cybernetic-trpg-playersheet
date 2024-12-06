use std::{path::PathBuf, rc::Rc};

use book::{
    class::{CharacterClass, Class},
    Book,
};
use gtk4::glib::ExitCode;
use non_empty_string::NonEmptyString;

mod book;
mod view;

fn main() -> ExitCode {
    let book = Rc::new({
        let arg = std::env::args_os().nth(1).map(PathBuf::from);
        let mut path = match arg.or_else(view::open_or_create_book) {
            Some(path) => path,
            // this means the user didn't pick a book to open.
            None => return ExitCode::SUCCESS,
        };

        Book::read(path)
    });

    return ExitCode::SUCCESS;
}
