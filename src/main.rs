#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use iced::{Settings, Application, window::{self, Position}};
use ui::LoadImages;
mod ui;
fn main() -> iced::Result {
    LoadImages::run(Settings {
        window:window::Settings{
            size:(1920,1080),
            position: Position::Centered,
            ..window::Settings::default()
        },
        ..Default::default()
    })?;

    Ok(())
}
