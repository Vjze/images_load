use iced::{
    widget::{button, center, column, container, mouse_area, opaque, stack, text, vertical_space},
    Color, Element, Length,
};

use crate::ui::Message;

// pub fn tip<'a>(state: bool, c: Container<'a, Message>, body: &'a str) -> Container<'a, Message> {
//     let overlay = if state {
//         Some(
//             Card::new(
//                 text("提示")
//                     .horizontal_alignment(iced::alignment::Horizontal::Center)
//                     .width(Length::Fill),
//                 text(format!("{}", body)).horizontal_alignment(iced::alignment::Horizontal::Center),
//             )
//             .foot(
//                 row![button(
//                     text("确认").horizontal_alignment(iced::alignment::Horizontal::Center),
//                 )
//                 .width(Length::Fill)
//                 .on_press(Message::CloseModal),]
//                 .spacing(10)
//                 .padding(5)
//                 .width(Length::Fill),
//             )
//             .max_width(300.0)
//             .on_close(Message::CloseModal),
//         )
//     } else {
//         None
//     };
//     let tip = modal(c, overlay)
//         .backdrop(Message::OpenModal)
//         .on_esc(Message::CloseModal)
//         .align_y(alignment::Vertical::Center);

//     container(tip).into()
// }
pub fn modal<'a>(state: bool, base: Element<'a, Message>, str: String) -> Element<'a, Message> {
    if state {
        let body = container(
            column![
                container(text("提示").size(24)).center_x(Length::Fill),
                column![
                    container(text(format!("{}", str)).size(18)).center_x(Length::Fill),
                    vertical_space(),
                    container(button(text("确认").size(16)).on_press(Message::CloseModal))
                        .center_x(Length::Fill)
                        .padding(10),
                ]
                .spacing(10)
            ]
            .spacing(25),
        )
        .width(300)
        .height(200).style(|_theme|{
            container::Style { background: Some(
                Color {
                    a: 1.,
                    ..Color::BLACK
                }
                .into(),
            ),
            ..container::Style::default() }
        });
        stack![
            base,
            opaque(
                mouse_area(center(opaque(body)).style(|_theme| {
                    container::Style {
                        background: Some(
                            Color {
                                a: 0.8,
                                ..Color::BLACK
                            }
                            .into(),
                        ),
                        ..container::Style::default()
                    }
                }))
                // .on_press(Message::CloseModal)
            )
        ]
        .into()
    } else {
        base.into()
    }
}
