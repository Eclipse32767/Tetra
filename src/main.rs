use iced::{Result, Application, Color, Settings};
use iced::widget::{Button, Row, Container};
use iced_aw::{color_picker, ColorPicker};
use iced_style::Theme;
use libstyle::col_from_str;
mod libstyle;

fn main() -> Result {
    Configurator::run(Settings::default())
}

struct Configurator {
    bg1: Color,
    bg2: Color,
    bg3: Color,
    txt: Color,
    red: Color,
    orange: Color,
    yellow: Color,
    green: Color,
    blue: Color,
    purple: Color,
    pink: Color,
    open_picker: Option<ColorSlot>
}

#[derive(Debug, Clone)]
enum Message {
    OpenPicker(ColorSlot),
    SubmitColor(Color),
    ClosePicker,
}
#[derive(Debug, Clone, PartialEq)]
enum ColorSlot {
    Bg1,
    Bg2,
    Bg3,
    Txt,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink
}
impl Default for Configurator {
    fn default() -> Self {
        Configurator { 
            bg1: col_from_str("181926"), 
            bg2: col_from_str("1e2030"), 
            bg3: col_from_str("24273a"), 
            txt: col_from_str("cad3f5"),
            red: col_from_str("ed8796"), 
            orange: col_from_str("f5a97f"), 
            yellow: col_from_str("eed49f"), 
            green: col_from_str("a6da95"),
            blue: col_from_str("8aadf4"), 
            purple: col_from_str("c6a0f6"),
            pink: col_from_str("f5bde6"), 
            open_picker: None 
        }
    }
}

impl Application for Configurator {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();
    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        (Configurator::default(), iced::Command::none())
    }
    fn title(&self) -> String {
        String::from("Counter app")
    }
    fn update(&mut self, message: Self::Message) -> iced::Command<Message> {
        match message {
            Message::OpenPicker(value) => self.open_picker = Some(value),
            Message::SubmitColor(value) => {
                match self.open_picker.as_ref().unwrap() {
                    ColorSlot::Bg1 => self.bg1 = value,
                    ColorSlot::Bg2 => self.bg2 = value,
                    ColorSlot::Bg3 => self.bg3 = value,
                    ColorSlot::Txt => self.txt = value,
                    ColorSlot::Red => self.red = value,
                    ColorSlot::Orange => self.orange = value,
                    ColorSlot::Yellow => self.yellow = value,
                    ColorSlot::Green => self.green = value,
                    ColorSlot::Blue => self.blue = value,
                    ColorSlot::Purple => self.purple = value,
                    ColorSlot::Pink => self.pink = value,
                }
            }
            Message::ClosePicker => self.open_picker = None,
        }
        iced::Command::none()
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {
        let bg1but = Button::new("bg1").on_press(Message::OpenPicker(ColorSlot::Bg1));
        let bg1picker = ColorPicker::new(match &self.open_picker {
            Some(value) => value == &ColorSlot::Bg1,
            None => false
        }, self.red, bg1but, Message::ClosePicker, Message::SubmitColor);
        Container::new(bg1picker).center_x().center_y().width(iced::Length::Fill).height(iced::Length::Fill).into()
    }
}