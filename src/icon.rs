// Generated automatically by iced_fontello at build time.
// Do not edit manually. Source: ../assets/fonts/fontello.toml
// 65e43d13a306940df4bca46a6200cc5901d80d364684d98cc166d5debed4f595
use iced::widget::{text, Text};
use iced::Font;

pub const FONT: &[u8] = include_bytes!("../assets/fonts/fontello.ttf");

pub fn edit<'a>() -> Text<'a> {
    icon("\u{270E}")
}

pub fn file<'a>() -> Text<'a> {
    icon("\u{F15C}")
}

pub fn filter<'a>() -> Text<'a> {
    icon("\u{F0B0}")
}

pub fn folder<'a>() -> Text<'a> {
    icon("\u{1F4C2}")
}

pub fn iconfile<'a>() -> Text<'a> {
    icon("\u{F1C5}")
}

pub fn info<'a>() -> Text<'a> {
    icon("\u{2139}")
}

pub fn magnifying<'a>() -> Text<'a> {
    icon("\u{F07E}")
}

pub fn palette<'a>() -> Text<'a> {
    icon("\u{1F3A8}")
}

pub fn resize<'a>() -> Text<'a> {
    icon("\u{E744}")
}

pub fn resize2<'a>() -> Text<'a> {
    icon("\u{E745}")
}

pub fn resize_height<'a>() -> Text<'a> {
    icon("\u{2B0C}")
}

pub fn resize_width<'a>() -> Text<'a> {
    icon("\u{2B0D}")
}

pub fn save<'a>() -> Text<'a> {
    icon("\u{1F4BE}")
}

pub fn search<'a>() -> Text<'a> {
    icon("\u{1F50D}")
}

pub fn settings<'a>() -> Text<'a> {
    icon("\u{26EF}")
}

pub fn text_cursor<'a>() -> Text<'a> {
    icon("\u{F246}")
}

pub fn trash<'a>() -> Text<'a> {
    icon("\u{E10A}")
}

pub fn video<'a>() -> Text<'a> {
    icon("\u{1F3F9}")
}

fn icon(codepoint: &str) -> Text<'_> {
    text(codepoint).font(Font::with_name("fontello"))
}
