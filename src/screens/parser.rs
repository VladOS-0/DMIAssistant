use iced::{widget::toggler, Element, Task};
use iced_aw::TabLabel;

use crate::{wrap, DMIAssistant, Message};

use super::Screen;

#[derive(Debug, Clone)]
pub enum ParserMessage {
    Toggled(bool),
}

#[derive(Default, Debug, Clone)]
pub struct ParserScreen {
    is_checked: bool,
}

impl Screen for ParserScreen {
    fn label(&self) -> TabLabel {
        TabLabel::IconText('\u{1F4C2}', " Parser".to_string())
    }

    fn update(app: &mut DMIAssistant, message: Message) -> Task<Message> {
        if let Message::ParserMessage(screen_message) = message {
            match screen_message {
                ParserMessage::Toggled(state) => {
                    app.parser_screen.is_checked = state;
                    Task::none()
                }
            }
        } else {
            Task::none()
        }
    }

    fn view(app: &DMIAssistant) -> Element<'_, Message> {
        toggler(app.parser_screen.is_checked)
            .label("Toggle me!")
            .on_toggle(|state| wrap![ParserMessage::Toggled(state)])
            .into()
    }
}
