use iced::widget::container;
use iced::window::{Event, Id};
use iced::{color, Background, Element, Task};
use iced::{Length, Theme};
use iced_aw::time_picker::Status;
use iced_aw::{tab_bar, Tabs};
use screens::parser::{ParserMessage, ParserScreen};

pub mod dmi_model;
pub mod dmi_utils;
pub mod screens;
pub mod utils;
pub mod widgets;

use crate::screens::Screen;
use screens::debugger::{DebuggerMessage, DebuggerScreen};
use screens::loader::{LoaderMessage, LoaderScreen};
use screens::Screens;
use utils::cleanup;

pub mod icon;

pub const DEFAULT_THEME: Theme = Theme::Nightfly;

#[derive(Debug, Clone)]
pub enum Message {
    Debug,
    Window(Id, Event),
    LoaderMessage(LoaderMessage),
    DebuggerMessage(DebuggerMessage),
    ParserMessage(ParserMessage),
    ChangeScreen(Screens),
}

#[derive(Default, Debug)]
pub struct DMIAssistant {
    pub current_screen: Screens,

    pub debugger_screen: DebuggerScreen,
    pub loader_screen: LoaderScreen,
    pub parser_screen: ParserScreen,

    pub theme: Theme,
}

impl DMIAssistant {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        if let Message::Window(_id, event) = &message {
            match event {
                Event::Closed | Event::CloseRequested => {
                    cleanup();
                    iced::exit()
                }
                _ => match self.current_screen {
                    Screens::Parser => ParserScreen::update(self, message),
                    Screens::Loader => LoaderScreen::update(self, message),
                    Screens::Debugger => DebuggerScreen::update(self, message),
                },
            }
        } else {
            match message {
                Message::ChangeScreen(screen) => {
                    self.current_screen = screen;
                    Task::none()
                }
                Message::LoaderMessage(msg) => {
                    LoaderScreen::update(self, Message::LoaderMessage(msg))
                }
                Message::ParserMessage(msg) => {
                    ParserScreen::update(self, Message::ParserMessage(msg))
                }
                Message::DebuggerMessage(msg) => {
                    DebuggerScreen::update(self, Message::DebuggerMessage(msg))
                }

                _ => Task::none(),
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(
            Tabs::new(Message::ChangeScreen)
                .tab_icon_position(iced_aw::tabs::Position::Left)
                .push(
                    Screens::Parser,
                    self.parser_screen.label(),
                    ParserScreen::view(self),
                )
                .push(
                    Screens::Loader,
                    self.loader_screen.label(),
                    LoaderScreen::view(self),
                )
                .push(
                    Screens::Debugger,
                    self.debugger_screen.label(),
                    DebuggerScreen::view(self),
                )
                .set_active_tab(&self.current_screen)
                .tab_label_spacing(20)
                .tab_bar_height(Length::Shrink)
                .tab_label_padding(10)
                .tab_bar_style(|_, status| match status {
                    Status::Active => tab_bar::Style {
                        tab_label_background: Background::Color(color!(
                            0x3447c7
                        )),
                        text_color: color!(0xffffff),
                        icon_color: color!(0xffffff),
                        ..Default::default()
                    },
                    Status::Hovered => tab_bar::Style {
                        tab_label_background: Background::Color(color!(
                            0x293cba
                        )),
                        text_color: color!(0xffffff),
                        icon_color: color!(0xffffff),
                        ..Default::default()
                    },
                    Status::Pressed => tab_bar::Style {
                        tab_label_background: Background::Color(color!(
                            0x132285
                        )),
                        text_color: color!(0xffffff),
                        icon_color: color!(0xffffff),
                        ..Default::default()
                    },
                    Status::Disabled => tab_bar::Style {
                        tab_label_background: Background::Color(color!(
                            0x132285
                        )),
                        text_color: color!(0xffffff),
                        icon_color: color!(0xffffff),
                        ..Default::default()
                    },
                    _ => tab_bar::Style {
                        tab_label_background: Background::Color(color!(
                            0x132285
                        )),
                        text_color: color!(0xffffff),
                        icon_color: color!(0xffffff),
                        ..Default::default()
                    },
                }),
        )
        .padding(10)
        .into()
    }
}
