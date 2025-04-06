use crate::app;
use iced::Element;
use iced::widget::{column, text};

#[derive(Default)]
pub struct TimersView;

impl TimersView {
    pub fn view(&self) -> Element<app::Message> {
        column![text!("Timers view")].into()
    }
}
