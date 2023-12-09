#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use iced::{Settings, Application, window::{self, Position}, Size};
use ui::LoadImages;
mod ui;
fn main() -> iced::Result {
    LoadImages::run(Settings {
        window:window::Settings{
            size:Size::new(1920.0,1080.0),
            position: Position::Centered,
            ..window::Settings::default()
        },
        ..Default::default()
    })?;

    Ok(())
}
