#![allow(dead_code)]
#![deny(unsafe_code)]
use iced_style::button;
use iced::{Color, Background};
use iced::theme::{self, Theme};
use serde_derive::{Serialize, Deserialize};

#[derive(Clone)]
pub struct ButtonStyle {
    pub border_radius: f32,
    pub txt_color: Color,
    pub bg_color: Option<Color>,
    pub border_color: Color,
    pub border_width: f32,
    pub shadow_offset: iced::Vector,
}

impl button::StyleSheet for ButtonStyle {
    type Style = Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance { 
            shadow_offset: self.shadow_offset, 
            background: match self.bg_color {
                Some(x) => Some(Background::Color(x.clone())),
                None => None
            }, 
            border_radius: self.border_radius.into(),
            border_width: self.border_width, 
            border_color: self.border_color.clone(), 
            text_color: self.txt_color.clone()
        }
    }
}

#[derive(Clone)]
pub struct ThemeSet {
    pub light: CustomTheme,
    pub dark: CustomTheme,
    pub custom: CustomTheme,
}

#[derive(Clone)]
pub struct CustomTheme {
    pub application: theme::Palette,
    pub secondary: ButtonStyle,
    pub sidebar: ButtonStyle,
}

impl ButtonStyle {
    pub fn mk_theme(&self) -> theme::Button {
        theme::Button::Custom(Box::new(self.clone()))
    }
}
pub fn mk_app_theme(palette: theme::Palette) -> Theme {
    Theme::Custom(Box::new(theme::Custom::new(palette)))
}

pub fn col_from_string(string: String) -> Color {
    let chars: Vec<char> = string.chars().into_iter().collect();
    let red_str = format!("{}{}", chars[0], chars[1]);
    let green_str = format!("{}{}", chars[2], chars[3]);
    let blue_str = format!("{}{}", chars[4], chars[5]);
    let red = u8::from_str_radix(&red_str, 16).unwrap();
    let green = u8::from_str_radix(&green_str, 16).unwrap();
    let blue = u8::from_str_radix(&blue_str, 16).unwrap();
    Color::from_rgb8(red, green, blue)
}
pub fn col_from_str(string: &str) -> Color {
    col_from_string(string.to_string())
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ThemeFile {
    pub bg_color1: String,
    pub bg_color2: String,
    pub bg_color3: String,
    pub txt_color: String,
    pub red: String,
    pub orange: String,
    pub yellow: String,
    pub green: String,
    pub blue: String,
    pub purple: String,
    pub pink: String
}