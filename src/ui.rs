use iced::{
    executor,
    widget::{button, column, container, row, text, text_input, Image, Space, tooltip, image::Handle},
    Application, Command, Length, Theme, Subscription, keyboard, Element, theme,
};

use crate::loadfile::{load, ImageInfo};


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
    FontLoaded(Result<(), iced::font::Error>),
}
impl Application for LoadImages {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let font_command = iced::font::load(include_bytes!("../resources/assets/SourceHanSansHWSC-Regular.otf").as_slice());
        (
            Self {
                images: Default::default(),
                num: 0,
                input: Default::default(),
                now: Default::default(),
            },
            font_command.map(Message::FontLoaded),
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
                if self.images.is_empty(){
                    Command::none()
                }else {
                    self.now = self.images.get(self.num.clone()).unwrap().clone();
                    self.num += 1;
                    Command::none()
                }
                
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
            Message::FontLoaded(_) => Command::none(),
        }
    }
    fn subscription(&self) -> Subscription<Message> {
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
            row!(Image::<Handle>::new(self.now.clone().path)
                .height(Length::Fill)
                .width(Length::Fill)
            )
            .align_items(iced::Alignment::Center),
        )
        .center_x()
        .center_y()
        .max_width(1920)
        .max_height(850); //.width(1920).height(1080);
        let isempty = self.images.len() != 0;
        let load_btn = action(text("打开文件夹"), "打开文件夹", true.then_some(Message::Load),100);
        let pre_btn = action(text("上一张"), "上一张", isempty.then_some(Message::Pre),50);
        let next_btn = action(text("下一张"), "下一张", isempty.then_some(Message::Next),50);
        let all = text(format!("一共:{} 张", self.images.clone().len()));
        let now = text(format!("第: {} 张", self.num.clone()));
        let flie_name = text(format!("{}", self.now.name)).width(500);
        let size = if self.images.is_empty() {
            text(format!("尺寸： X "))
        }else{
            let img = image::open(self.now.clone().path)
                        .expect("Error loading cat image")
                        .into_rgba8();
            text(format!("尺寸：{} X {}",img.width().clone(),img.height().clone()))
        };
        
        let input = text_input("placeholder", self.input.to_string().as_str())
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
        .align_items(iced::Alignment::End)
        .spacing(50).padding(15);
        let col = column!(image_view, Space::new(Length::Fill, Length::Fill), row).padding(10);
          
        container(col).center_x().padding(5).height(Length::Fill).into()
    }
}

fn action<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    label: &'a str,
    on_press: Option<Message>,
    width:u16,
) -> Element<'a, Message> {
    let action = button(container(content).width(width).center_x());

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