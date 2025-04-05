use crate::{timer, ui};
use iced::Element;

#[derive(Default)]
pub struct Pomodoro {
    /// Initial configuration for timer
    timer_settings: timer::Settings,
    /// Current timer state
    timer_state: timer::Settings,
    ui: ui::PomodoroUi,
}

#[derive(Clone, Copy, Debug)]
pub enum Message {
    Setup(timer::Settings),
    Tick,
    Reset,
    UiUpdate(ui::Message),
}

impl From<timer::Settings> for Pomodoro {
    fn from(settings: timer::Settings) -> Self {
        let mut pomodoro = Self::default();

        pomodoro.update(Message::Setup(settings));

        pomodoro
    }
}

impl Pomodoro {
    /// ## Initial timer configuration setup.
    ///
    /// This function resets the current timer to match the new initial configuration provided.
    fn setup_timer_settings(&mut self, settings: timer::Settings) {
        self.timer_settings = settings;

        self.update(Message::Reset);
    }

    fn reset_timer(&mut self) {
        self.timer_state = self.timer_settings;
    }

    fn tick_and_continue(time_left: &mut u64) -> bool {
        if *time_left > 0 {
            *time_left -= 1;
            return true;
        }

        false
    }

    fn tick(&mut self) {
        let study_time = &mut self.timer_state.time_to_study;

        if Self::tick_and_continue(study_time) {
            return;
        }

        let relax_time = &mut self.timer_state.time_to_relax;

        if Self::tick_and_continue(relax_time) {
            return;
        }
    }

    pub fn get_timer_state(&self) -> timer::Settings {
        self.timer_state
    }

    pub fn get_timer_config(&self) -> timer::Settings {
        self.timer_settings
    }

    pub fn view(&self) -> Element<Message> {
        self.ui.view().into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Setup(settings) => self.setup_timer_settings(settings),
            Message::Tick => self.tick(),
            Message::Reset => self.reset_timer(),
            Message::UiUpdate(message) => self.ui.update(message),
        }
    }
}
