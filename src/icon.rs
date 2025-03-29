// Generated automatically by iced_fontello at build time.
// Do not edit manually. Source: ../assets/fonts/fontello.toml
// d6bb8e29153df2fe67d93d4dd0ee218222d372a8ab376a3fbcf1dcfb5e3eae1f
use iced::widget::{text, Text};
use iced::Font;

pub const FONT: &[u8] = include_bytes!("../assets/fonts/fontello.ttf");

pub fn edit<'a>() -> Text<'a> {
    icon("\u{270E}")
}

pub fn file_explorer<'a>() -> Text<'a> {
    icon("\u{1F4C2}")
}

pub fn open<'a>() -> Text<'a> {
    icon("\u{F15C}")
}

pub fn palette<'a>() -> Text<'a> {
    icon("\u{1F3A8}")
}

pub fn save<'a>() -> Text<'a> {
    icon("\u{1F4BE}")
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

fn icon(codepoint: &str) -> Text<'_> {
    text(codepoint).font(Font::with_name("fontello"))
}
