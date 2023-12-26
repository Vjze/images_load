use iced::{
    alignment,
    widget::{button, container, row, text, Container},
    Length,
};
use iced_aw::{modal, Card};

use super::ui::Message;

pub fn tip<'a>(state: bool, c: Container<'a, Message>, body: &'a str) -> Container<'a, Message> {
    let overlay = if state {
        Some(
            Card::new(
                text("提示")
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                    .width(Length::Fill),
                text(format!("{}", body)).horizontal_alignment(iced::alignment::Horizontal::Center),
            )
            .foot(
                row![button(
                    text("确认").horizontal_alignment(iced::alignment::Horizontal::Center),
                )
                .width(Length::Fill)
                .on_press(Message::CloseModal),]
                .spacing(10)
                .padding(5)
                .width(Length::Fill),
            )
            .max_width(300.0)
            .on_close(Message::CloseModal),
        )
    } else {
        None
    };
    let tip = modal(c, overlay)
        .backdrop(Message::OpenModal)
        .on_esc(Message::CloseModal)
        .align_y(alignment::Vertical::Center);

    container(tip).into()
}

