#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use iced::{
    window::{self, Position},
    Application, Font, Settings, Size,
};
use ui::LoadImages;
mod ui;
mod loadfile;
mod view;
mod tip;
fn main() -> iced::Result {
    let icon = window::icon::from_file_data(
        include_bytes!("../resources/logo/icons.png"),
        Some(image::ImageFormat::Png),
    )
    .ok();
    LoadImages::run(Settings {
        window: window::Settings {
            icon,
            // size: Size::new(1920.0, 1080.0),
            size: Size::new(1600.0, 900.0),
            // min_size: Some(Size::new(1600.0, 900.0)),
            position: Position::Centered,
            ..window::Settings::default()
        },
        default_font: Font::with_name("Source Han Sans HW SC"),
        ..Default::default()
    })?;

    Ok(())
}
