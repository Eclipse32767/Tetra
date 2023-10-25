#![deny(unsafe_code)]
use iced::{Result, Application, Color, Settings, Length};
use iced::widget::{Button, Column, Row, Container, Text, Space};
use iced_aw::ColorPicker;
use iced_style::Theme;
use gettextrs::*;
use iced_style::theme::Palette;
use oceania_style::{ButtonStyle, get_home, get_set_theme, ListStyle, make_custom_theme, MenuStyle, mk_app_theme, SelectedTheme, string_to_color, ThemeCustom, ThemeFile, ThemeSet};
use serde_derive::{Serialize, Deserialize};
use toml;



pub fn col_from_str(string: &str) -> Color {
    string_to_color(string.to_string())
}
fn main() -> Result {
    let _ = textdomain("TetraTheme");
    let _ = bind_textdomain_codeset("TetraTheme", "UTF-8");
    Configurator::run(Settings::default())
}
const COLOR_SIZE: u16 = 50;

fn button_style_from_col(color: &Color) -> ButtonStyle{
    ButtonStyle { border_radius: 2.5, txt_color: color.clone(), bg_color: color.clone(), border_color: color.clone(), border_width: 0.0, shadow_offset: iced::Vector { x: 0.0, y: 0.0 } }
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
    open_picker: Option<ColorSlot>,
    theme_set: ThemeSet,
    theme_type: SelectedTheme,
}
#[derive(Serialize, Deserialize)]
struct CuttlefishCfg {
    theme: String
}

fn generate_theme(theme_slot: SelectedTheme) -> Option<ThemeCustom> {
    match theme_slot {
        SelectedTheme::Light => {
            Some(ThemeCustom {
                application: Palette {
                    background: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                    text: Color::from_rgb8(0x00, 0x19, 0x36),
                    primary: Color::from_rgb8(0x00, 0x77, 0xFF),
                    success: Color::from_rgb8(0x00, 0xCB, 0x40),
                    danger: Color::from_rgb8(0xFF, 0x4C, 0x00),
                },
                sidebar: ButtonStyle {
                    border_radius: 2.0,
                    txt_color: Color::from_rgb8(0x00, 0x19, 0x36),
                    bg_color: Color::from_rgb8(0xD2, 0xF0, 0xFF),
                    border_color: Color::from_rgb8(0, 0, 0),
                    border_width: 0.0,
                    shadow_offset: iced::Vector { x: 0.0, y: 0.0 }
                },
                secondary: ButtonStyle {
                    border_radius: 2.0,
                    txt_color: Color::from_rgb8(0x00, 0x20, 0x46),
                    bg_color: Color::from_rgb8(0xC6, 0xEC, 0xFF),
                    border_color: Color::from_rgb8(0, 0, 0),
                    border_width: 0.0,
                    shadow_offset: iced::Vector { x: 0.0, y: 0.0 }
                },
                list: ListStyle {
                    txt_color: Color::from_rgb8(0x00, 0x19, 0x36),
                    bg_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                    handle_color: Color::from_rgb8(0x00, 0x19, 0x36),
                    border_radius: 5.0,
                    border_width: 2.0,
                    border_color: Color::from_rgb8(0x00, 0x19, 0x36),
                    menu: MenuStyle {
                        txt_color: Color::from_rgb8(0x00, 0x19, 0x36),
                        bg_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                        border_radius: 5.0,
                        border_width: 2.0,
                        border_color: Color::from_rgb8(0x00, 0x19, 0x36),
                        sel_txt_color: Color::from_rgb8(0x00, 0x19, 0x36),
                        sel_bg_color: Color::from_rgb8(0x00, 0xF1, 0xD6),
                    }
                }
            })
        }
        SelectedTheme::Dark => {
            Some(ThemeCustom { // TODO: set dark theme properly
                application: Palette {
                    background: Color::from_rgb8(0x00, 0x19, 0x36),
                    text: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                    primary: Color::from_rgb8(0x00, 0xAB, 0xE1),
                    success: Color::from_rgb8(0x00, 0xA9, 0x35),
                    danger: Color::from_rgb8(0xC5, 0x3A, 0x00),
                },
                sidebar: ButtonStyle {
                    border_radius: 2.0,
                    txt_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                    bg_color: Color::from_rgb8(0x00, 0x20, 0x46),
                    border_color: Color::from_rgb8(0, 0, 0),
                    border_width: 0.0,
                    shadow_offset: iced::Vector { x: 0.0, y: 0.0 }
                },
                secondary: ButtonStyle {
                    border_radius: 2.0,
                    txt_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                    bg_color: Color::from_rgb8(0x00, 0x29, 0x58),
                    border_color: Color::from_rgb8(0, 0, 0),
                    border_width: 0.0,
                    shadow_offset: iced::Vector { x: 0.0, y: 0.0 }
                },
                list: ListStyle {
                    txt_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                    bg_color: Color::from_rgb8(0x00, 0x29, 0x58),
                    handle_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                    border_radius: 5.0,
                    border_width: 2.0,
                    border_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                    menu: MenuStyle {
                        txt_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                        bg_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                        border_radius: 5.0,
                        border_width: 2.0,
                        border_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                        sel_txt_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                        sel_bg_color: Color::from_rgb8(0x00, 0xCD, 0xB6),
                    }
                }
            })
        }
        SelectedTheme::Custom => {
            None
        }
    }
}

fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}
fn string_from_col(color: &Color) -> String {
    let rgba = color.into_rgba8();
    let red_str = format_radix(rgba[0].into(), 16);
    let green_str = format_radix(rgba[1].into(), 16);
    let blue_str = format_radix(rgba[2].into(), 16);
    format!("{red_str}{green_str}{blue_str}")
}

#[derive(Debug, Clone)]
enum Message {
    OpenPicker(ColorSlot),
    SubmitColor(Color),
    ClosePicker,
    Save
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
        let path = format!("{}/Oceania/theme.toml", get_home());
        let theme_file: Option<ThemeFile> = match std::fs::read_to_string(path) {
            Ok(value) => {
                Some(toml::from_str(&value).unwrap())
            }
            Err(..) => {
                None
            }
        };
        match theme_file {
            Some(value) => {
                Configurator {
                    bg1: string_to_color(value.bg_color1.clone()),
                    bg2: string_to_color(value.bg_color2.clone()),
                    bg3: string_to_color(value.bg_color3.clone()),
                    txt: string_to_color(value.txt_color.clone()),
                    red: string_to_color(value.red.clone()),
                    orange: string_to_color(value.orange),
                    yellow: string_to_color(value.yellow),
                    green: string_to_color(value.green.clone()),
                    blue: string_to_color(value.blue.clone()),
                    purple: string_to_color(value.purple),
                    pink: string_to_color(value.pink),
                    open_picker: None,
                    theme_type: get_set_theme(),
                    theme_set: ThemeSet {
                        light: ThemeCustom {
                            application: Palette {
                                background: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                                text: Color::from_rgb8(0x00, 0x19, 0x36),
                                primary: Color::from_rgb8(0x00, 0x77, 0xFF),
                                success: Color::from_rgb8(0x00, 0xCB, 0x40),
                                danger: Color::from_rgb8(0xFF, 0x4C, 0x00),
                            },
                            sidebar: ButtonStyle {
                                border_radius: 2.0,
                                txt_color: Color::from_rgb8(0x00, 0x19, 0x36),
                                bg_color: Color::from_rgb8(0xD2, 0xF0, 0xFF),
                                border_color: Color::from_rgb8(0, 0, 0),
                                border_width: 0.0,
                                shadow_offset: iced::Vector { x: 0.0, y: 0.0 }
                            },
                            secondary: ButtonStyle {
                                border_radius: 2.0,
                                txt_color: Color::from_rgb8(0x00, 0x20, 0x46),
                                bg_color: Color::from_rgb8(0xC6, 0xEC, 0xFF),
                                border_color: Color::from_rgb8(0, 0, 0),
                                border_width: 0.0,
                                shadow_offset: iced::Vector { x: 0.0, y: 0.0 }
                            },
                            list: ListStyle {
                                txt_color: Color::from_rgb8(0x00, 0x19, 0x36),
                                bg_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                                handle_color: Color::from_rgb8(0x00, 0x19, 0x36),
                                border_radius: 5.0,
                                border_width: 2.0,
                                border_color: Color::from_rgb8(0x00, 0x19, 0x36),
                                menu: MenuStyle {
                                    txt_color: Color::from_rgb8(0x00, 0x19, 0x36),
                                    bg_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                                    border_radius: 5.0,
                                    border_width: 2.0,
                                    border_color: Color::from_rgb8(0x00, 0x19, 0x36),
                                    sel_txt_color: Color::from_rgb8(0x00, 0x19, 0x36),
                                    sel_bg_color: Color::from_rgb8(0x00, 0xF1, 0xD6),
                                }
                            }
                        },
                        dark: ThemeCustom { // TODO: set dark theme properly
                            application: Palette {
                                background: Color::from_rgb8(0x00, 0x19, 0x36),
                                text: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                                primary: Color::from_rgb8(0x00, 0xAB, 0xE1),
                                success: Color::from_rgb8(0x00, 0xA9, 0x35),
                                danger: Color::from_rgb8(0xC5, 0x3A, 0x00),
                            },
                            sidebar: ButtonStyle {
                                border_radius: 2.0,
                                txt_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                                bg_color: Color::from_rgb8(0x00, 0x20, 0x46),
                                border_color: Color::from_rgb8(0, 0, 0),
                                border_width: 0.0,
                                shadow_offset: iced::Vector { x: 0.0, y: 0.0 }
                            },
                            secondary: ButtonStyle {
                                border_radius: 2.0,
                                txt_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                                bg_color: Color::from_rgb8(0x00, 0x29, 0x58),
                                border_color: Color::from_rgb8(0, 0, 0),
                                border_width: 0.0,
                                shadow_offset: iced::Vector { x: 0.0, y: 0.0 }
                            },
                            list: ListStyle {
                                txt_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                                bg_color: Color::from_rgb8(0x00, 0x29, 0x58),
                                handle_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                                border_radius: 5.0,
                                border_width: 2.0,
                                border_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                                menu: MenuStyle {
                                    txt_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                                    bg_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                                    border_radius: 5.0,
                                    border_width: 2.0,
                                    border_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                                    sel_txt_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                                    sel_bg_color: Color::from_rgb8(0x00, 0xCD, 0xB6),
                                }
                            }
                        },
                        custom: make_custom_theme()
                    },
                }
            }
            None => {
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
                    open_picker: None,
                    theme_type: get_set_theme(),
                    theme_set: ThemeSet { 
                        light: generate_theme(SelectedTheme::Light).unwrap(),
                        dark: generate_theme(SelectedTheme::Dark).unwrap(),
                        custom: make_custom_theme(),
                    }
                }
            }
        }
    }
}

impl Application for Configurator {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();
    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        (Configurator::default(), iced::Command::none())
    }
    fn title(&self) -> String {
        gettext("Tetra Theme Tool")
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
            Message::Save => {
                let file = ThemeFile {
                    bg_color1: string_from_col(&self.bg1),
                    bg_color2: string_from_col(&self.bg2),
                    bg_color3: string_from_col(&self.bg3),
                    txt_color: string_from_col(&self.txt),
                    red: string_from_col(&self.red),
                    orange: string_from_col(&self.orange),
                    yellow: string_from_col(&self.yellow),
                    green: string_from_col(&self.green),
                    blue: string_from_col(&self.blue),
                    purple: string_from_col(&self.purple),
                    pink: string_from_col(&self.pink),
                };
                let path = format!("{}/Oceania/theme.toml", get_home());
                let toml_out = toml::to_string(&file).unwrap();
                let _ = std::fs::write(path, toml_out);
            }
        }
        iced::Command::none()
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {
        let bg1_but = Button::new("").on_press(Message::OpenPicker(ColorSlot::Bg1)).width(COLOR_SIZE).height(COLOR_SIZE).style(button_style_from_col(&self.bg1).mk_theme());
        let bg1_picker = ColorPicker::new(match &self.open_picker {
            Some(value) => value == &ColorSlot::Bg1,
            None => false
        }, self.bg1, bg1_but, Message::ClosePicker, Message::SubmitColor);
        let bg1label = Text::new(gettext("Primary Background Color"));
        let bg1_row = Row::new().push(bg1label).push(Space::new(Length::Fill, 10)).push(bg1_picker).align_items(iced::Alignment::Center);
        let bg2_but = Button::new("").on_press(Message::OpenPicker(ColorSlot::Bg2)).width(COLOR_SIZE).height(COLOR_SIZE).style(button_style_from_col(&self.bg2).mk_theme());
        let bg2_picker = ColorPicker::new(match &self.open_picker {
            Some(value) => value == &ColorSlot::Bg2,
            None => false
        }, self.bg2, bg2_but, Message::ClosePicker, Message::SubmitColor);
        let bg2label = Text::new(gettext("Secondary Background Color"));
        let bg2_row = Row::new().push(bg2label).push(Space::new(Length::Fill, 10)).push(bg2_picker).align_items(iced::Alignment::Center).spacing(10);
        let bg3_but = Button::new("").on_press(Message::OpenPicker(ColorSlot::Bg3)).width(COLOR_SIZE).height(COLOR_SIZE).style(button_style_from_col(&self.bg3).mk_theme());
        let bg3_picker = ColorPicker::new(match &self.open_picker {
            Some(value) => value == &ColorSlot::Bg3,
            None => false
        }, self.bg3, bg3_but, Message::ClosePicker, Message::SubmitColor);
        let bg3label = Text::new(gettext("Tertiary Background Color"));
        let bg3_row = Row::new().push(bg3label).push(Space::new(Length::Fill, 10)).push(bg3_picker).align_items(iced::Alignment::Center).spacing(10);
        let txt_but = Button::new("").on_press(Message::OpenPicker(ColorSlot::Txt)).width(COLOR_SIZE).height(COLOR_SIZE).style(button_style_from_col(&self.txt).mk_theme());
        let txt_picker = ColorPicker::new(match &self.open_picker {
            Some(value) => value == &ColorSlot::Txt,
            None => false
        }, self.txt, txt_but, Message::ClosePicker, Message::SubmitColor);
        let txt_label = Text::new(gettext("Text Color"));
        let txt_row = Row::new().push(txt_label).push(Space::new(Length::Fill, 10)).push(txt_picker).align_items(iced::Alignment::Center).spacing(10);
        let red_but = Button::new("").on_press(Message::OpenPicker(ColorSlot::Red)).width(COLOR_SIZE).height(COLOR_SIZE).style(button_style_from_col(&self.red).mk_theme());
        let red_picker = ColorPicker::new(match &self.open_picker {
            Some(value) => value == &ColorSlot::Red,
            None => false
        }, self.red, red_but, Message::ClosePicker, Message::SubmitColor);
        let red_label = Text::new(gettext("Red Color"));
        let red_row = Row::new().push(red_label).push(Space::new(Length::Fill, 10)).push(red_picker).align_items(iced::Alignment::Center).spacing(10);
        let orange_but = Button::new("").on_press(Message::OpenPicker(ColorSlot::Orange)).width(COLOR_SIZE).height(COLOR_SIZE).style(button_style_from_col(&self.orange).mk_theme());
        let orange_picker = ColorPicker::new(match &self.open_picker {
            Some(value) => value == &ColorSlot::Orange,
            None => false
        }, self.orange, orange_but, Message::ClosePicker, Message::SubmitColor);
        let orange_label = Text::new(gettext("Orange Color"));
        let orange_row = Row::new().push(orange_label).push(Space::new(Length::Fill, 10)).push(orange_picker).align_items(iced::Alignment::Center).spacing(10);
        let yellow_but = Button::new("").on_press(Message::OpenPicker(ColorSlot::Yellow)).width(COLOR_SIZE).height(COLOR_SIZE).style(button_style_from_col(&self.yellow).mk_theme());
        let yellow_picker = ColorPicker::new(match &self.open_picker {
            Some(value) => value == &ColorSlot::Yellow,
            None => false
        }, self.yellow, yellow_but, Message::ClosePicker, Message::SubmitColor);
        let yellow_label = Text::new(gettext("Yellow Color"));
        let yellow_row = Row::new().push(yellow_label).push(Space::new(Length::Fill, 10)).push(yellow_picker).align_items(iced::Alignment::Center).spacing(10);
        let green_but = Button::new("").on_press(Message::OpenPicker(ColorSlot::Green)).width(COLOR_SIZE).height(COLOR_SIZE).style(button_style_from_col(&self.green).mk_theme());
        let green_picker = ColorPicker::new(match &self.open_picker {
            Some(value) => value == &ColorSlot::Green,
            None => false
        }, self.green, green_but, Message::ClosePicker, Message::SubmitColor);
        let green_label = Text::new(gettext("Green Color"));
        let green_row = Row::new().push(green_label).push(Space::new(Length::Fill, 10)).push(green_picker).align_items(iced::Alignment::Center).spacing(10);
        let blue_but = Button::new("").on_press(Message::OpenPicker(ColorSlot::Blue)).width(COLOR_SIZE).height(COLOR_SIZE).style(button_style_from_col(&self.blue).mk_theme());
        let blue_picker = ColorPicker::new(match &self.open_picker {
            Some(value) => value == &ColorSlot::Blue,
            None => false
        }, self.blue, blue_but, Message::ClosePicker, Message::SubmitColor);
        let blue_label = Text::new(gettext("Blue Color"));
        let blue_row = Row::new().push(blue_label).push(Space::new(Length::Fill, 10)).push(blue_picker).align_items(iced::Alignment::Center).spacing(10);
        let purple_but = Button::new("").on_press(Message::OpenPicker(ColorSlot::Purple)).width(COLOR_SIZE).height(COLOR_SIZE).style(button_style_from_col(&self.purple).mk_theme());
        let purple_picker = ColorPicker::new(match &self.open_picker {
            Some(value) => value == &ColorSlot::Purple,
            None => false
        }, self.purple, purple_but, Message::ClosePicker, Message::SubmitColor);
        let purple_label = Text::new(gettext("Purple Color"));
        let purple_row = Row::new().push(purple_label).push(Space::new(Length::Fill, 10)).push(purple_picker).align_items(iced::Alignment::Center).spacing(10);
        let pink_but = Button::new("").on_press(Message::OpenPicker(ColorSlot::Pink)).width(COLOR_SIZE).height(COLOR_SIZE).style(button_style_from_col(&self.pink).mk_theme());
        let pink_picker = ColorPicker::new(match &self.open_picker {
            Some(value) => value == &ColorSlot::Pink,
            None => false
        }, self.pink, pink_but, Message::ClosePicker, Message::SubmitColor);
        let pink_label = Text::new(gettext("Pink Color"));
        let pink_row = Row::new().push(pink_label).push(Space::new(Length::Fill, 10)).push(pink_picker).align_items(iced::Alignment::Center).spacing(10);
        let save = Button::new(Text::new(gettext("Save"))).on_press(Message::Save);
        let save_row = Row::new().align_items(iced::Alignment::Start).push(save).push(Space::new(Length::Fill, 10));

        let master = Column::new().push(bg1_row).push(bg2_row).push(bg3_row).push(txt_row).push(red_row).push(orange_row).push(yellow_row).push(green_row).push(blue_row).push(purple_row).push(pink_row).push(save_row).align_items(iced::Alignment::Center).spacing(10);
        Container::new(master).center_x().center_y().width(Length::Fill).height(Length::Fill).into()
    }
    fn theme(&self) -> Self::Theme {
        match self.theme_type {
            SelectedTheme::Light => mk_app_theme(self.theme_set.light.application),
            SelectedTheme::Dark => mk_app_theme(self.theme_set.dark.application),
            SelectedTheme::Custom => mk_app_theme(self.theme_set.custom.application),
        }
    }
}