use iced::widget;
use playersheet::PlayerSheet;

mod playersheet;
mod psheet_io;

fn main() -> Result<(), Box<dyn core::error::Error>> {
    iced::application::<(), u8, iced::Theme, iced::Renderer>(
        |_state| "create or open".to_string(),
        |_state, message| match message {
            0 => iced::Task::none(),
            1 => iced::Task::none(),
            _ => iced::Task::none(),
        },
        |_state| {
            iced::widget::row![
                iced::widget::button("Open").on_press(0),
                iced::widget::button("Create").on_press(1),
            ]
            .into()
        },
    )
    .settings(iced::Settings::default())
    .run_with(|| ((), iced::Task::none()))
    .unwrap();

    todo!();

    let state = init_from_arg()
        .or_else(init_from_dialog)
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Player sheet not selected",
        ))?;
    iced::application(title, update, view).run_with(|| (state, iced::Task::none()))?;
    Ok(())
}

fn init_from_arg() -> Option<State> {
    let path = std::env::args_os().nth(1)?;
    let playersheet = psheet_io::read(&path).ok()?;
    Some(State {
        path: path.into(),
        playersheet,
    })
}

fn init_from_dialog() -> Option<State> {
    let path = rfd::FileDialog::new()
        .set_title("Open or create player sheet")
        .add_filter(psheet_io::EXTENSION, psheet_io::EXTENSIONS)
        .set_file_name("Stranger")
        .save_file()?;
    let playersheet = psheet_io::read(&path).ok()?;
    Some(State { path, playersheet })
}

struct State {
    path: std::path::PathBuf,
    playersheet: PlayerSheet,
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
