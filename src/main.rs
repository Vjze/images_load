#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use iced::{
    advanced::graphics::image::image_rs::ImageFormat,
    window::{self}, Font, Size,
};
use ui::LoadImages;
mod loadfile;
mod tip;
mod ui;
mod view;
fn main() -> iced::Result {
    let icon = window::icon::from_file_data(
        include_bytes!("../resources/logo/icons.png"),
        Some(ImageFormat::from_mime_type("image/png").unwrap()),
    )
    .ok();
    iced::application("Image-Loader", LoadImages::update, LoadImages::view)
        .font(include_bytes!("../resources/assets/SourceHanSansHWSC-Regular.otf").as_slice())
        .centered()
        .default_font(Font::with_name("Source Han Sans HW SC"))
        .window_size(Size::new(1600.0, 900.0))
        .subscription(LoadImages::subscription)
        .window(window::Settings {
            icon,
            ..Default::default()
        })
        .run()
}
