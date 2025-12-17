use super::app::{Message, Pomodoro};
use iced::{Subscription, time};

/// Subscription handler to tick the internal state
pub fn ticker(pomodoro: &Pomodoro) -> Subscription<Message> {
    if pomodoro.is_running() {
        time::every(time::seconds(1)).map(|_| Message::Tick)
    } else {
        Subscription::none()
    }
}
