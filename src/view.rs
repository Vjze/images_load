use crate::ui::Message;
use iced::widget::{center, image};
use iced::{
    widget::{image::Handle, text, Column},
    Alignment, Element, Length,
};
use iced_aw::Spinner;

pub static IMG_LOGO: &[u8] = include_bytes!("../resources/logo/icons.png");

pub fn load_message<'a>() -> Element<'a, Message> {
    center(image(Handle::from_bytes(IMG_LOGO)))
        .width(Length::Fill)
        .height(Length::Fill)
        
        .into()
}
pub fn loading_message<'a>() -> Element<'a, Message> {
    let spinner = Spinner::new().circle_radius(2.0).width(Length::Fill);
    let col = Column::new()
        .push(spinner)
        .push(text("正在查找图片....请稍后").size(35))
        .align_x(Alignment::Center)
        .spacing(30)
        .padding(10)
        .width(450);
    center(col)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
