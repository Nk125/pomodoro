use crate::app;
use iced::Element;
use iced::widget::{button, column, row, text};

#[derive(Default)]
pub struct PomodoroUi {
    current_view: View,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum View {
    #[default]
    Setup,
    Timers,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ChangeView(View),
}

impl PomodoroUi {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChangeView(view) => self.current_view = view,
        }
    }

    pub fn view(&self) -> Element<app::Message> {
        let setup_button =
            button("Setup").on_press(app::Message::UiUpdate(Message::ChangeView(View::Setup)));

        let timers_button =
            button("Timers").on_press(app::Message::UiUpdate(Message::ChangeView(View::Timers)));

        column![
            row![setup_button, timers_button].spacing(20),
            text!("{:?}", self.current_view)
        ]
        .into()
    }
}
