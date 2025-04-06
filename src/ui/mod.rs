mod views;

use crate::app;
use iced::Element;
use iced::widget::{button, column, row};

#[derive(Default)]
pub struct PomodoroUi {
    current_view: views::View,
    setup_view: views::setup::SetupView,
    timers_view: views::timers::TimersView,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ChangeView(views::View),
}

impl PomodoroUi {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChangeView(view) => self.current_view = view,
        }
    }

    pub fn view(&self) -> Element<app::Message> {
        let setup_button = button("Setup").on_press(app::Message::UiUpdate(Message::ChangeView(
            views::View::Setup,
        )));

        let timers_button = button("Timers").on_press(app::Message::UiUpdate(Message::ChangeView(
            views::View::Timers,
        )));

        let current_view_rendered = match self.current_view {
            views::View::Setup => self.setup_view.view(),
            views::View::Timers => self.timers_view.view(),
        };

        column![
            row![setup_button, timers_button].spacing(20),
            current_view_rendered
        ]
        .into()
    }
}
