#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dmi_assistant::{
    icon::FONT,
    utils::{cleanup, init_temp},
    DMIAssistant, Message, DEFAULT_THEME,
};
use iced::{
    advanced::graphics::image::image_rs::ImageFormat,
    font,
    window::{self, icon::from_file_data},
    Font, Size, Subscription,
};

pub fn main() -> iced::Result {
    cleanup();
    init_temp();
    iced::application("DMI assistant", DMIAssistant::update, DMIAssistant::view)
        .theme(|_| DEFAULT_THEME)
        .subscription(subscription)
        .settings(iced::Settings {
            default_font: Font::MONOSPACE,
            default_text_size: 14.into(),
            antialiasing: true,
            ..Default::default()
        })
        .window(window::Settings {
            size: Size::new(1500.0, 900.0),
            decorations: true,
            icon: from_file_data(
                include_bytes!("../assets/images/icon.png"),
                Some(ImageFormat::Png),
            )
            .ok(),
            exit_on_close_request: false,
            ..Default::default()
        })
        .font(FONT)
        .font(iced_fonts::REQUIRED_FONT_BYTES)
        .run()
}

fn subscription(_state: &DMIAssistant) -> Subscription<Message> {
    window::events().map(|(id, event)| Message::Window(id, event))
}

pub fn settings() -> iced::Settings {
    iced::Settings {
        default_font: font::Font::MONOSPACE,
        default_text_size: 18.0.into(),
        antialiasing: true,
        ..Default::default()
    }
}
