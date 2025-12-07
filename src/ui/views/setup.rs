use crate::app;
use iced::Element;
use iced::widget::{column, text};

#[derive(Default)]
pub struct SetupView;

impl SetupView {
    pub fn view(&self) -> Element<'_, app::Message> {
        column![text!("Setup view")].into()
    }
}
