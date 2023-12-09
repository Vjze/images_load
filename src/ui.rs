use iced::{
    executor,
    widget::{button, column, container, image, row, text, text_input, Image, Space, tooltip},
    Application, Command, Length, Theme, Subscription, keyboard, Element, theme,
};

#[derive(Debug, Clone, Default)]
pub struct ImageInfo {
    path: String,
    name: String,
}
pub struct LoadImages {
    images: Vec<ImageInfo>,
    now: ImageInfo,
    num: usize,
    input: String,
}
#[derive(Debug, Clone)]
pub enum Message {
    Load,
    LoadCallback(Vec<ImageInfo>),
    Next,
    InputChange(String),
    Pre,
}
impl Application for LoadImages {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                images: Default::default(),
                num: 0,
                input: Default::default(),
                now: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Image-Loader".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Load => Command::perform(load(), Message::LoadCallback),
            Message::Next => {
                let now = self.num.clone();
                let input = if self.input.len() == 0 {
                    0
                } else {
                    self.input.clone().parse::<usize>().unwrap()
                };
                if now != input && input != 0 {
                    self.now = self.images.clone().get(input).unwrap().clone();
                    self.num = input;
                } else if now >= self.images.len()  {
                    self.now = self.images.clone().get(0).unwrap().clone();
                    self.num = 1 ;
                }
                else {
                    self.now = self.images.clone().get(now).unwrap().clone();
                    self.num += 1;
                }
                self.input.clear();
                Command::none()
            }
            Message::LoadCallback(list) => {
                self.images = list;
                self.now = self.images.get(self.num.clone()).unwrap().clone();
                self.num += 1;
                Command::none()
            }
            Message::InputChange(i) => {
                self.input = i;
                Command::none()
            }
            Message::Pre => {
                let mut now = self.num.clone();
                let input = if self.input.len() == 0 {
                    0
                } else {
                    self.input.clone().parse::<usize>().unwrap()
                };
                if now != input && input != 0 {
                    self.now = self.images.clone().get(input).unwrap().clone();
                    self.num = input;
                } else if now <= 1 {
                    now = self.images.len();
                    self.now = self.images.clone().get(now - 1).unwrap().clone();
                    self.num = now;
                }else {
                    self.now = self.images.clone().get(now - 2).unwrap().clone();
                    let _ = self.num -= 1;
                }
                self.input.clear();
                Command::none()
            }
        }
    }
    fn subscription(&self) -> Subscription<Message> {
            // iced::subscription::events_with(|event, _| {
            //     if let iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
            //         key_code,
            //         modifiers,
            //     }) = event
            //     {
            //         if key_code == iced::keyboard::KeyCode::Down && modifiers.is_empty() {
            //             return Some(Message::Next);
            //         }
            //         if key_code == iced::keyboard::KeyCode::Up && modifiers.is_empty() {
            //             return Some(Message::Pre);
            //         }if key_code == iced::keyboard::KeyCode::Right && modifiers.is_empty() {
            //             return Some(Message::Next);
            //         }if key_code == iced::keyboard::KeyCode::Left && modifiers.is_empty() {
            //             return Some(Message::Pre);
            //         }
            //     }
            //     None
            // })
            keyboard::on_key_press(|key_code, modifiers| match key_code {
                keyboard::KeyCode::Down if modifiers.is_empty() => {
                    Some(Message::Next)
                }
                keyboard::KeyCode::Right if modifiers.is_empty() => {
                    Some(Message::Next)
                }
                keyboard::KeyCode::Left if modifiers.is_empty() => {
                    Some(Message::Pre)
                }
                keyboard::KeyCode::Up if modifiers.is_empty() => {
                    Some(Message::Pre)
                }
                _ => None,
            })
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let image_view = container(
            row!(Image::<image::Handle>::new(self.now.clone().path)
                .height(930)
                .width(1920))
            .align_items(iced::Alignment::Center),
        )
        .center_x(); //.width(1920).height(1080);
        let isempty = self.images.len() != 0;
        let load_btn = action(text("Load"), "Load", true.then_some(Message::Load));;
        let pre_btn = action(text("Pre"), "Pre", isempty.then_some(Message::Pre));
        let next_btn = action(text("Next"), "Next", isempty.then_some(Message::Next));
        let all = text(format!("all :{}", self.images.clone().len()));
        let now = text(format!("now : {}", self.num.clone()));
        let flie_name = text(format!("{}", self.now.name)).width(500);
        let input = text_input("placeholder", self.input.to_string().as_str())
            .on_input(Message::InputChange);
    
        let row = row!(
            load_btn,
            pre_btn,
            input,
            next_btn,
            Space::new(Length::Fill, Length::Shrink),
            flie_name,
            now,
            all,
            
            // Space::new(Length::Fill, Length::Shrink)
            
        )
        .align_items(iced::Alignment::End)
        .spacing(50).padding(15);
        let col = column!(image_view, Space::new(Length::Fill, Length::Shrink), row);
          
        container(col).center_x().padding(10).into()
    }
}
async fn load() -> Vec<ImageInfo> {
    let mut list = vec![];
    let paths = rfd::AsyncFileDialog::new().pick_folder().await.unwrap();
    for path in walkdir::WalkDir::new(paths.path().display().to_string())
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let file_name = path.file_name().to_str().unwrap().to_string();
        let file_path = path.path().display().to_string();
        list.push(ImageInfo {
            path: file_path,
            name: file_name,
        });
    }
    list
}
fn action<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    label: &'a str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let action = button(container(content).width(30).center_x());

    if let Some(on_press) = on_press {
        tooltip(
            action.on_press(on_press),
            label,
            tooltip::Position::FollowCursor,
        )
        .style(theme::Container::Box)
        .into()
    } else {
        action.style(theme::Button::Secondary).into()
    }
}