use iced::widget;
use playersheet::PlayerSheet;

mod playersheet;

fn main() -> Result<(), Box<dyn core::error::Error>> {
    let (playersheet, path) = playersheet::io::read_arg_or_create_or_open_dialog()?;
    iced::application(title, update, view)
        .run_with(|| (State { path, playersheet }, iced::Task::none()))?;
    Ok(())
}

pub fn eframe_test() {
    eframe::run_simple_native(
        "Want to open or create a player sheet?",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size(egui::Vec2 { x: 250.0, y: 250.0 }),
            vsync: true,
            multisampling: 0,
            depth_buffer: 0,
            stencil_buffer: 0,
            hardware_acceleration: eframe::HardwareAcceleration::Off,
            renderer: eframe::Renderer::default(),
            run_and_return: true,
            event_loop_builder: None,
            window_builder: None,
            shader_version: None,
            centered: true,
            persist_window: false,
            persistence_path: None,
            dithering: true,
        },
        |context, _frame| {
            egui::CentralPanel::default().show(context, |ui| {
                if ui.add(egui::Button::new("Open")).clicked() {
                    context.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                if ui.add(egui::Button::new("Create")).clicked() {
                    context.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        },
    )
    .ok();
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
