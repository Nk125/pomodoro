use env_logger::Env;
use pomodoro::{app, clock};

fn main() -> iced::Result {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    iced::application(
        app::Pomodoro::default,
        app::Pomodoro::update,
        app::Pomodoro::view,
    )
    .subscription(clock::ticker)
    .antialiasing(true)
    .run()
}
