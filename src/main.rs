use env_logger::Env;
use pomodoro::app;

fn main() -> iced::Result {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    iced::application(
        app::Pomodoro::default,
        app::Pomodoro::update,
        app::Pomodoro::view,
    )
    .antialiasing(true)
    .run()
}
