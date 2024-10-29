use iced::widget;
use sheet::Character;

mod dialog;
mod sheet;

fn main() -> Result<(), Box<dyn core::error::Error>> {
    let (playersheet, path) = dialog::open_or_create()?;
    iced::application(title, update, view)
        .run_with(|| (State { path, playersheet }, iced::Task::none()))?;
    Ok(())
}

struct State {
    path: std::path::PathBuf,
    playersheet: Character,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} @ {}",
            self.playersheet.name,
            self.path.display()
        ))
    }
}

#[derive(Debug, Clone)]
enum Message {}

fn title(state: &State) -> String {
    state.to_string()
}

fn update(_state: &mut State, _message: Message) {}

fn view(state: &State) -> widget::Column<Message> {
    widget::Column::new().push(widget::text(
        serde_json::to_string(&state.playersheet).unwrap(),
    ))
}

/*
pub fn read_arg() -> Option<(Character, std::path::PathBuf)> {
    let mut path = std::env::args_os().nth(1).map(std::path::PathBuf::from)?;
    let sheet = io::read(&mut path).ok()?;
    Some((sheet, path))
}

pub fn read_arg_or_create_or_open_dialog() -> Result<(Character, std::path::PathBuf), io::Error> {
    if let Some(values) = read_arg() {
        return Ok(values);
    }
    dialog::open_or_create()
}
*/
