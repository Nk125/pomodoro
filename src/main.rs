use pomodoro::app;

fn main() -> iced::Result {
    iced::application("Pomodoro", app::Pomodoro::update, app::Pomodoro::view)
        .centered()
        .run()
}
