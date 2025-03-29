use iced::{Element, Task};
use iced_aw::TabLabel;

use crate::{DMIAssistant, Message};

pub mod debugger;
pub mod loader;
pub mod parser;

/// Wrapping a screen's Message into the app's Message. Screen's message enum and variant in app's message enum must have the
/// same name.
/// ```
/// use dmi_assistant::screens::debugger::DebuggerMessage;
/// use dmi_assistant::Message;
/// use dmi_assistant::wrap;
///
/// // equals to Message::DebuggerMessage(DebuggerMessage::LoadDMI)
/// wrap![DebuggerMessage::LoadDMI];
/// ```
#[macro_export]
macro_rules! wrap {
    [$message:ident::$message2:ident] => {
        $crate::Message::$message($message::$message2)
    };
    [$message:ident::$message2:ident($($message_inner:expr),*)] => {
        $crate::Message::$message($message::$message2($($message_inner),*))
    };
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Screens {
    #[default]
    Parser,
    Loader,
    Debugger,
}

pub trait Screen {
    fn label(&self) -> TabLabel;
    fn update(_app: &mut DMIAssistant, _message: Message) -> Task<Message> {
        Task::none()
    }
    fn view(app: &DMIAssistant) -> Element<'_, Message>;
}
