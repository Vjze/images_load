use iced::{
    keyboard, widget::{
        button, center, column, container, image::Handle, row, text, text_input, tooltip, Image, Space
    }, Element, Length, Subscription, Task, Theme
};

use crate::{
    loadfile::{load, Error, ImageInfo},
    tip::modal,
    view::{load_message, loading_message},
};

#[derive(Debug, Clone, Default)]
pub enum LoadImages {
    #[default]
    Load,
    Loading,
    Loaded(State),
}
#[derive(Debug, Default, Clone)]
pub struct State {
    images: Vec<ImageInfo>,
    now: ImageInfo,
    num: usize,
    input: String,
    tip: bool,
    errs: Error,
}
#[derive(Debug, Clone)]
pub enum Message {
    Load,
    Loaded(Result<Vec<ImageInfo>, Error>),
    Next,
    InputChange(String),
    Pre,
    CloseModal,
    OpenModal
}
impl LoadImages {
    
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match self {
            LoadImages::Load => match message {
                Message::Load => {
                    *self = LoadImages::Loading;
                    Task::perform(load(), Message::Loaded)
                }
                Message::Loaded(Ok(state)) => {
                    *self = LoadImages::Loaded(State {
                        images: state.clone(),
                        now: state.clone().get(0).unwrap().clone(),
                        num: 1,
                        ..State::default()
                    });
                    Task::none()
                }
                Message::Loaded(Err(e)) => {
                    *self = LoadImages::Loaded(State {
                        tip: true,
                        errs: e,
                        ..Default::default()
                    });
                    Task::none()
                }
                _ => Task::none(),
            },
            LoadImages::Loading => match message {
                Message::Loaded(Ok(state)) => {
                    *self = LoadImages::Loaded(State {
                        images: state.clone(),
                        now: state.clone().get(0).unwrap().clone(),
                        num: 1,
                        ..State::default()
                    });
                    Task::none()
                }
                Message::Loaded(Err(e)) => {
                    *self = LoadImages::Loaded(State {
                        tip: true,
                        errs: e,
                        ..Default::default()
                    });
                    Task::none()
                }
                _ => Task::none(),
            },
            LoadImages::Loaded(state) => {
                let command = match message {
                    Message::Load => Task::perform(load(), Message::Loaded),
                    Message::Next => {
                        let now = state.num.clone();
                        let input = if state.input.len() == 0 {
                            0
                        } else {
                            state.input.clone().parse::<usize>().unwrap()
                        };
                        if now != input && input != 0 {
                            state.now = state.images.clone().get(input).unwrap().clone();
                            state.num = input;
                        } else if now >= state.images.len() {
                            state.now = state.images.clone().get(0).unwrap().clone();
                            state.num = 1;
                        } else {
                            state.now = state.images.clone().get(now).unwrap().clone();

                            state.num += 1;
                        }
                        state.input.clear();
                        Task::none()
                    }
                    Message::Loaded(Ok(list)) => {
                        *self = LoadImages::Loaded(State {
                            images: list.clone(),
                            now: list.clone().get(0).unwrap().clone(),
                            num: 1,
                            ..State::default()
                        });
                        Task::none()
                    }
                    Message::InputChange(i) => {
                        state.input = i;
                        Task::none()
                    }
                    Message::Pre => {
                        let mut now = state.num.clone();
                        let input = if state.input.len() == 0 {
                            0
                        } else {
                            state.input.clone().parse::<usize>().unwrap()
                        };
                        if now != input && input != 0 {
                            state.now = state.images.clone().get(input).unwrap().clone();
                            state.num = input;
                        } else if now <= 1 {
                            now = state.images.len();
                            state.now = state.images.clone().get(now - 1).unwrap().clone();
                            state.num = now;
                        } else {
                            state.now = state.images.clone().get(now - 2).unwrap().clone();
                            let _ = state.num -= 1;
                        }
                        state.input.clear();
                        Task::none()
                    }
                    Message::Loaded(Err(e)) => {
                        state.tip = true;
                        state.errs = e;
                        Task::none()
                    }
                    Message::OpenModal => {
                        state.tip = true;
                        Task::none()
                    }
                    Message::CloseModal => {
                        state.tip = false;
                        Task::none()
                    }
                };
                Task::batch(vec![command])
            }
        }
    }
    pub fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key_code, modifiers| match key_code {
            keyboard::Key::Named(keyboard::key::Named::ArrowDown) if modifiers.is_empty() => Some(Message::Next),
            keyboard::Key::Named(keyboard::key::Named::ArrowRight) if modifiers.is_empty() => Some(Message::Next),
            keyboard::Key::Named(keyboard::key::Named::ArrowUp) if modifiers.is_empty() => Some(Message::Pre),
            keyboard::Key::Named(keyboard::key::Named::ArrowLeft) if modifiers.is_empty() => Some(Message::Pre),
            _ => None,
        })
    }

    pub fn view(&self) -> Element<Message> {
        let load_btn = action(
            text("打开文件夹"),
            "打开文件夹",
            true.then_some(Message::Load),
            100,
        );

        let view = match self {
            LoadImages::Load => {
                let view = load_message();
                let col = column!(view, load_btn).align_x(iced::Alignment::Center);
                center(col).padding(15).into()
            }
            LoadImages::Loaded(state) => {
                match state.images.is_empty() {
                    true => {
                        let view = load_message();
                        let col = column!(view, load_btn).align_x(iced::Alignment::Center);
                        let body = match state.errs {
                            Error::DialogClosed => "选择框已关闭",
                            Error::ListNone => "当前目录下没有找到图片!!!",
                        };
                        let all_tip = modal(state.tip, container(col).into(), body.to_string());
                        center(all_tip).padding(15).into()
                    }
                    false => {
                        let img = image::open(state.now.clone().path).unwrap().into_rgba8();
                        let (width, height) = img.dimensions();
                        let imgs = Image::new(Handle::from_rgba(width, height, img.into_raw()));
                        let image_view = center(
                            row!(imgs.height(Length::Fill).width(Length::Fill))
                                .align_y(iced::Alignment::Center),
                        )
                        .max_width(1920)
                        .max_height(900); 
                        let isempty = state.images.len() != 0;
                        let load_btn = action(
                            text("打开文件夹"),
                            "打开文件夹",
                            true.then_some(Message::Load),
                            100,
                        );
                        let pre_btn = action(
                            text("上一张"),
                            "上一张",
                            isempty.then_some(Message::Pre),
                            50,
                        );
                        let next_btn = action(
                            text("下一张"),
                            "下一张",
                            isempty.then_some(Message::Next),
                            50,
                        );
                        let all = text(format!("一共:{} 张", state.images.clone().len()));
                        let now = text(format!("第: {} 张", state.num.clone()));
                        let flie_name = text(format!("{}", state.now.name)).width(500);
                        let size = if state.images.is_empty() {
                            text(format!("尺寸： X "))
                        } else {
                            text(format!("尺寸：{} X {}", width, height))
                        };

                        let input = text_input("placeholder", state.input.to_string().as_str())
                            .on_input(Message::InputChange);

                        let row = row!(
                            load_btn,
                            pre_btn,
                            input,
                            next_btn,
                            Space::new(Length::Fill, Length::Shrink),
                            flie_name,
                            size,
                            now,
                            all,
                        )
                        .align_y(iced::Alignment::End)
                        .spacing(50)
                        .padding(15);

                        let col = column!(image_view, Space::new(Length::Fill, Length::Fill), row);
                        let body = match state.errs {
                            Error::DialogClosed => "选择框已关闭",
                            Error::ListNone => "当前目录下没有找到图片!!!",
                        };
                        let all_tip = modal(state.tip, container(col).into(), body.to_string());
                        center(all_tip)
                            .padding(5)
                            .height(Length::Fill)
                            .into()
                    }
                }
            }
            LoadImages::Loading => loading_message(),
        };
        view
    }
}

fn action<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    label: &'a str,
    on_press: Option<Message>,
    width: u16,
) -> Element<'a, Message> {
    let action = button(container(content).width(width).center_x(Length::Fill));

    if let Some(on_press) = on_press {
        tooltip(
            action.on_press(on_press),
            label,
            tooltip::Position::FollowCursor,
        )
        // .style(theme::Container::Box)
        .into()
    } else {
        action.into()
    }
}
