use iced::widget::container;
use iced::Element;
use iced::Length;
use iced::Task;
use iced_aw::TabLabel;

use crate::DMIAssistant;
use crate::Message;

use super::Screen;

#[derive(Debug, Clone)]
pub enum LoaderMessage {
    Debug,
}

#[derive(Default, Debug, Clone)]
pub struct LoaderScreen {}

impl Screen for LoaderScreen {
    fn label(&self) -> TabLabel {
        TabLabel::IconText('\u{F15C}', " Loader".to_string())
    }

    fn update(_app: &mut DMIAssistant, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(_app: &DMIAssistant) -> Element<'_, Message> {
        container("todo!")
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
