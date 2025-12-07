use crate::{timer, ui};
use iced::Element;

#[derive(Default)]
pub struct Pomodoro {
    /// Initial timer configuration.
    /// Used as the base state when performing a reset.
    timer_settings: timer::Settings,

    /// Current running state of the timer.
    /// This value decreases as time progresses.
    timer_state: timer::Settings,

    /// UI manager for rendering and handling interface updates.
    ui: ui::PomodoroUi,
}

#[derive(Clone, Copy, Debug)]
/// Messages emitted either by the UI or the timer system.
pub enum Message {
    /// Applies a new initial timer configuration.
    Setup(timer::Settings),

    /// Advances the timer by one tick (one second).
    Tick,

    /// Resets the timer back to its initial configuration.
    Reset,

    /// Forwards UI-related messages to the UI subsystem.
    UiUpdate(ui::Message),
}

impl From<timer::Settings> for Pomodoro {
    fn from(settings: timer::Settings) -> Self {
        let mut pomodoro = Self::default();

        // Initialize timer settings using a Setup message.
        pomodoro.update(Message::Setup(settings));

        pomodoro
    }
}

impl Pomodoro {
    /// Applies a new initial timer configuration and resets the active timer
    /// to match the newly provided settings.
    fn setup_timer_settings(&mut self, settings: timer::Settings) {
        self.timer_settings = settings;
        self.update(Message::Reset);
    }

    /// Restores the active timer state to the initial configuration.
    fn reset_timer(&mut self) {
        self.timer_state = self.timer_settings;
    }

    /// Decrements the given counter by one second.
    ///
    /// Returns `true` if the counter continues running after decrementing,
    /// or `false` if the counter has reached zero.
    #[inline]
    fn tick_and_continue(time_left: &mut u64) -> bool {
        match time_left {
            t if *t > 0 => {
                *t -= 1;
                true
            }
            _ => false,
        }
    }

    /// Advances the timer forward by a single second.
    ///
    /// The study timer is advanced first. Once it reaches zero,
    /// the relaxation timer begins to tick.
    /// If both have reached zero, nothing happens.
    fn tick(&mut self) {
        if Self::tick_and_continue(&mut self.timer_state.time_to_study) {
            return;
        }

        let _ = Self::tick_and_continue(&mut self.timer_state.time_to_relax);
    }

    /// Returns the current active state of the timer.
    #[must_use]
    pub fn timer_state(&self) -> timer::Settings {
        self.timer_state
    }

    /// Returns the initial timer configuration.
    #[must_use]
    pub fn timer_config(&self) -> timer::Settings {
        self.timer_settings
    }

    // == UI Integration ==

    /// Returns the UI view corresponding to the current state.
    pub fn view(&self) -> Element<'_, Message> {
        self.ui.view()
    }

    /// Processes incoming messages and updates the pomodoro state accordingly.
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Setup(settings) => self.setup_timer_settings(settings),
            Message::Tick => self.tick(),
            Message::Reset => self.reset_timer(),
            Message::UiUpdate(msg) => self.ui.update(msg),
        }
    }
}
