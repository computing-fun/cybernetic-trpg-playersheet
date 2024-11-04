use book::Book;
use gtk4::glib::ExitCode;
use rfd::FileDialog;
use sheet::{Character, Class, Race};

mod book;
mod sheet;
mod view;

fn main() -> ExitCode {
    let book_path = match std::env::args_os().nth(1) {
        Some(path) => path.to_string_lossy().to_string(),
        None => match FileDialog::new()
            .set_title("Which book do you want to pick up?")
            .add_filter("Book", &["book"])
            .pick_file()
        {
            Some(path) => path,
            None => return ExitCode::SUCCESS,
        }
        .to_string_lossy()
        .to_string(),
    };

    let book = match Book::open(book_path) {
        Ok(book) => book,
        Err(err) => {
            view::error(err);
            return ExitCode::FAILURE;
        }
    };

    while let Some(menu_opt) = view::main_menu() {
        if let Err(err) = run_menu_opt(menu_opt, &book) {
            view::error(&*err);
        }
    }

    ExitCode::SUCCESS
}

fn run_menu_opt(
    menu_opt: view::MainMenuOptions,
    book: &Book,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(match menu_opt {
        view::MainMenuOptions::NewCharacter => view::debug(book.read::<Character>("Unknown")?),
        view::MainMenuOptions::NewRace => view::debug(book.read::<Race>("Unknown")?),
        view::MainMenuOptions::NewClass => view::debug(book.read::<Class>("Unknown")?),
        view::MainMenuOptions::Tester => {
            book.write(&Character::default())?;
        }
    })
}
