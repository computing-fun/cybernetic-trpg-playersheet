#![allow(dead_code)]

use std::{path::PathBuf, process::ExitCode, rc::Rc};

mod book;
mod dialog;
mod view;

fn main() -> ExitCode {
    let book = Rc::new({
        match get_book() {
            Some(book_result) => match book_result {
                Ok(book) => book,
                Err(err) => {
                    dialog::error(err);
                    return ExitCode::FAILURE;
                }
            },
            None => return ExitCode::SUCCESS,
        }
    });

    view::book::full(book);

    return ExitCode::SUCCESS;
}

enum BookPath {
    Arg(PathBuf),
    Dialog(dialog::OpenOrCreateBook),
}

fn get_book_path() -> Option<BookPath> {
    if let Some(path) = std::env::args_os().nth(1).map(PathBuf::from) {
        return Some(BookPath::Arg(path));
    }

    if let Some(path) = dialog::open_or_create_book() {
        return Some(BookPath::Dialog(path));
    }

    return None;
}

fn get_book() -> Option<Result<book::Book, book::ZipBookError>> {
    match get_book_path()? {
        BookPath::Arg(path_buf) => Some(book::Book::try_from(path_buf)),
        BookPath::Dialog(open_or_create_book) => match open_or_create_book {
            dialog::OpenOrCreateBook::Open(path_buf) => Some(book::Book::try_from(path_buf)),
            dialog::OpenOrCreateBook::Create(path_buf) => Some(book::write_default_book(&path_buf)),
        },
    }
}
