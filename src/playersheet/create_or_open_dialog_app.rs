use std::sync::{Arc, Mutex};

use iced::{widget, Alignment, Element, Error, Length, Size, Task};

type State = Arc<Mutex<Message>>;

#[derive(Debug, Default, Clone, Copy)]
pub(crate) enum Message {
    #[default]
    None,
    Create,
    Open,
}

pub(crate) fn run() -> Result<Option<Message>, Error> {
    let state = Arc::new(Mutex::new(Message::default()));
    application(Arc::clone(&state))?;
    Ok(state.lock().ok().map(|m| m.to_owned()))
}

fn application(state: Arc<Mutex<Message>>) -> Result<(), iced::Error> {
    iced::application(title, update, view)
        .centered()
        .resizable(false)
        .window_size(Size::new(250.0, 100.0))
        .run_with(|| (state, Task::none()))
}

fn title(_state: &State) -> String {
    "Player Sheet".to_string()
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match state.lock() {
        Ok(mut guard) => {
            *guard = message;
            iced::exit()
        }
        Err(_) => Task::none(),
    }
}

fn view(_state: &State) -> Element<Message> {
    widget::column![
        widget::row![widget::text("Want to open or create a player sheet?")],
        widget::Space::new(Length::Fill, 10.0),
        widget::row![
            widget::button("Open").on_press(Message::Open),
            widget::Space::new(10.0, Length::Fill),
            widget::button("Create").on_press(Message::Create),
        ]
    ]
    .padding(5.0)
    .align_x(Alignment::Center)
    .width(Length::Fill)
    .into()
}
