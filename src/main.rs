#![deny(unsafe_code)]
use iced::{Result, Application, Color, Settings, Length};
use iced::widget::{Button, Column, Row, Container, Text, Space};
use iced_aw::ColorPicker;
use iced_style::Theme;
use gettextrs::*;
use libstyle::{col_from_str, ButtonStyle};
mod libstyle;

fn main() -> Result {
    let _ = textdomain("TetraTheme");
    let _ = bind_textdomain_codeset("TetraTheme", "UTF-8");
    Configurator::run(Settings::default())
}
const COLOR_SIZE: u16 = 50;

fn buttonstyle_from_col(color: &Color) -> ButtonStyle{
    ButtonStyle { border_radius: 2.5, txt_color: color.clone(), bg_color: Some(color.clone()), border_color: color.clone(), border_width: 0.0, shadow_offset: iced::Vector { x: 0.0, y: 0.0 } }
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
        let bg1but = Button::new("").on_press(Message::OpenPicker(ColorSlot::Bg1)).width(COLOR_SIZE).height(COLOR_SIZE).style(buttonstyle_from_col(&self.bg1).mk_theme());
        let bg1picker = ColorPicker::new(match &self.open_picker {
            Some(value) => value == &ColorSlot::Bg1,
            None => false
        }, self.bg1, bg1but, Message::ClosePicker, Message::SubmitColor);
        let bg1label = Text::new(gettext("Primary Background Color"));
        let bg1row = Row::new().push(bg1label).push(Space::new(Length::Fill, 10)).push(bg1picker).align_items(iced::Alignment::Center);
        let bg2but = Button::new("").on_press(Message::OpenPicker(ColorSlot::Bg2)).width(COLOR_SIZE).height(COLOR_SIZE).style(buttonstyle_from_col(&self.bg2).mk_theme());
        let bg2picker = ColorPicker::new(match &self.open_picker {
            Some(value) => value == &ColorSlot::Bg2,
            None => false
        }, self.bg2, bg2but, Message::ClosePicker, Message::SubmitColor);
        let bg2label = Text::new(gettext("Secondary Background Color"));
        let bg2row = Row::new().push(bg2label).push(Space::new(Length::Fill, 10)).push(bg2picker).align_items(iced::Alignment::Center).spacing(10);
        let bg3but = Button::new("").on_press(Message::OpenPicker(ColorSlot::Bg3)).width(COLOR_SIZE).height(COLOR_SIZE).style(buttonstyle_from_col(&self.bg3).mk_theme());
        let bg3picker = ColorPicker::new(match &self.open_picker {
            Some(value) => value == &ColorSlot::Bg3,
            None => false
        }, self.bg3, bg3but, Message::ClosePicker, Message::SubmitColor);
        let bg3label = Text::new(gettext("Tertiary Background Color"));
        let bg3row = Row::new().push(bg3label).push(Space::new(Length::Fill, 10)).push(bg3picker).align_items(iced::Alignment::Center).spacing(10);
        let txtbut = Button::new("").on_press(Message::OpenPicker(ColorSlot::Txt)).width(COLOR_SIZE).height(COLOR_SIZE).style(buttonstyle_from_col(&self.txt).mk_theme());
        let txtpicker = ColorPicker::new(match &self.open_picker {
            Some(value) => value == &ColorSlot::Txt,
            None => false
        }, self.txt, txtbut, Message::ClosePicker, Message::SubmitColor);
        let txtlabel = Text::new(gettext("Text Color"));
        let txtrow = Row::new().push(txtlabel).push(Space::new(Length::Fill, 10)).push(txtpicker).align_items(iced::Alignment::Center).spacing(10);
        let redbut = Button::new("").on_press(Message::OpenPicker(ColorSlot::Red)).width(COLOR_SIZE).height(COLOR_SIZE).style(buttonstyle_from_col(&self.red).mk_theme());
        let redpicker = ColorPicker::new(match &self.open_picker {
            Some(value) => value == &ColorSlot::Red,
            None => false
        }, self.red, redbut, Message::ClosePicker, Message::SubmitColor);
        let redlabel = Text::new(gettext("Red Color"));
        let redrow = Row::new().push(redlabel).push(Space::new(Length::Fill, 10)).push(redpicker).align_items(iced::Alignment::Center).spacing(10);
        let orangebut = Button::new("").on_press(Message::OpenPicker(ColorSlot::Orange)).width(COLOR_SIZE).height(COLOR_SIZE).style(buttonstyle_from_col(&self.orange).mk_theme());
        let orangepicker = ColorPicker::new(match &self.open_picker {
            Some(value) => value == &ColorSlot::Orange,
            None => false
        }, self.orange, orangebut, Message::ClosePicker, Message::SubmitColor);
        let orangelabel = Text::new(gettext("Orange Color"));
        let orangerow = Row::new().push(orangelabel).push(Space::new(Length::Fill, 10)).push(orangepicker).align_items(iced::Alignment::Center).spacing(10);
        let yellowbut = Button::new("").on_press(Message::OpenPicker(ColorSlot::Yellow)).width(COLOR_SIZE).height(COLOR_SIZE).style(buttonstyle_from_col(&self.yellow).mk_theme());
        let yellowpicker = ColorPicker::new(match &self.open_picker {
            Some(value) => value == &ColorSlot::Yellow,
            None => false
        }, self.yellow, yellowbut, Message::ClosePicker, Message::SubmitColor);
        let yellowlabel = Text::new(gettext("Yellow Color"));
        let yellowrow = Row::new().push(yellowlabel).push(Space::new(Length::Fill, 10)).push(yellowpicker).align_items(iced::Alignment::Center).spacing(10);
        let greenbut = Button::new("").on_press(Message::OpenPicker(ColorSlot::Green)).width(COLOR_SIZE).height(COLOR_SIZE).style(buttonstyle_from_col(&self.green).mk_theme());
        let greenpicker = ColorPicker::new(match &self.open_picker {
            Some(value) => value == &ColorSlot::Green,
            None => false
        }, self.green, greenbut, Message::ClosePicker, Message::SubmitColor);
        let greenlabel = Text::new(gettext("Green Color"));
        let greenrow = Row::new().push(greenlabel).push(Space::new(Length::Fill, 10)).push(greenpicker).align_items(iced::Alignment::Center).spacing(10);
        let bluebut = Button::new("").on_press(Message::OpenPicker(ColorSlot::Blue)).width(COLOR_SIZE).height(COLOR_SIZE).style(buttonstyle_from_col(&self.blue).mk_theme());
        let bluepicker = ColorPicker::new(match &self.open_picker {
            Some(value) => value == &ColorSlot::Blue,
            None => false
        }, self.blue, bluebut, Message::ClosePicker, Message::SubmitColor);
        let bluelabel = Text::new(gettext("Blue Color"));
        let bluerow = Row::new().push(bluelabel).push(Space::new(Length::Fill, 10)).push(bluepicker).align_items(iced::Alignment::Center).spacing(10);
        let purplebut = Button::new("").on_press(Message::OpenPicker(ColorSlot::Purple)).width(COLOR_SIZE).height(COLOR_SIZE).style(buttonstyle_from_col(&self.purple).mk_theme());
        let purplepicker = ColorPicker::new(match &self.open_picker {
            Some(value) => value == &ColorSlot::Purple,
            None => false
        }, self.purple, purplebut, Message::ClosePicker, Message::SubmitColor);
        let purplelabel = Text::new(gettext("Purple Color"));
        let purplerow = Row::new().push(purplelabel).push(Space::new(Length::Fill, 10)).push(purplepicker).align_items(iced::Alignment::Center).spacing(10);
        let pinkbut = Button::new("").on_press(Message::OpenPicker(ColorSlot::Pink)).width(COLOR_SIZE).height(COLOR_SIZE).style(buttonstyle_from_col(&self.pink).mk_theme());
        let pinkpicker = ColorPicker::new(match &self.open_picker {
            Some(value) => value == &ColorSlot::Pink,
            None => false
        }, self.pink, pinkbut, Message::ClosePicker, Message::SubmitColor);
        let pinklabel = Text::new(gettext("Pink Color"));
        let pinkrow = Row::new().push(pinklabel).push(Space::new(Length::Fill, 10)).push(pinkpicker).align_items(iced::Alignment::Center).spacing(10);

        let master = Column::new().push(bg1row).push(bg2row).push(bg3row).push(txtrow).push(redrow).push(orangerow).push(yellowrow).push(greenrow).push(bluerow).push(purplerow).push(pinkrow).align_items(iced::Alignment::Center).spacing(10);
        Container::new(master).center_x().center_y().width(iced::Length::Fill).height(iced::Length::Fill).into()
    }
}