use std::{path::PathBuf, rc::Rc};

use book::Book;
use gtk4::glib::ExitCode;

mod book;
mod sheet;
mod view;

fn main() -> ExitCode {
    let book = Rc::new({
        let arg = std::env::args_os().nth(1).map(PathBuf::from);
        let mut path = match arg.or_else(view::open_or_create_book) {
            Some(path) => path,
            // this means the user didn't pick a book to open.
            None => return ExitCode::SUCCESS,
        };
        if !path.set_extension("book") {
            return ExitCode::FAILURE;
        }
        match Book::open(path) {
            Ok(book) => book,
            Err(err) => {
                view::error(err);
                return ExitCode::FAILURE;
            }
        }
    });

    while let Some(menu_opt) = view::main_menu(Rc::clone(&book)) {
        match menu_opt {
            view::MainMenuResult::CharacterEditor(character) => {}
            view::MainMenuResult::RaceEditor(race) => {}
            view::MainMenuResult::ClassEditor(class) => {}
            view::MainMenuResult::CyberneticEditor(cybernetic) => {
                view::cybernetics_editor(Rc::clone(&book), cybernetic);
            }
        }
    }

    return ExitCode::SUCCESS;
}
