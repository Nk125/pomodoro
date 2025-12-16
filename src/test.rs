use crate::{app::*, timer};

#[test]
fn setup_settings() {
    let mut pomodoro = Pomodoro::default();

    let timer_state = pomodoro.timer_state();

    assert_eq!(timer_state.time_to_study, 0);
    assert_eq!(timer_state.time_to_relax, 0);

    let new_settings = timer::Settings {
        time_to_study: 50,
        time_to_relax: 10,
    };

    pomodoro.update(Message::Setup(new_settings));

    assert_eq!(pomodoro.timer_state(), &new_settings);
}

#[test]
fn timer_underflow() {
    let zero_timer_state = timer::Settings {
        time_to_study: 0,
        time_to_relax: 0,
    };

    let mut pomodoro = Pomodoro::from(zero_timer_state);

    pomodoro.update(Message::Tick);

    assert_eq!(pomodoro.timer_state(), &zero_timer_state);
}

#[test]
fn timer_reset() {
    let arbitrary_timer_settings = timer::Settings {
        time_to_study: 10,
        time_to_relax: 10,
    };

    let mut pomodoro = Pomodoro::from(arbitrary_timer_settings);

    pomodoro.update(Message::Tick);

    assert_ne!(pomodoro.timer_state(), &arbitrary_timer_settings);

    pomodoro.update(Message::Reset);

    assert_eq!(pomodoro.timer_state(), &arbitrary_timer_settings);
}

#[test]
fn timer_finish() {
    let mut pomodoro: Pomodoro = timer::Settings {
        time_to_study: 1,
        time_to_relax: 1,
    }
    .into();

    // Since both timers are still running, then study time hasn't finished yet
    assert_eq!(pomodoro.timer_state().state(), timer::State::StudyTime);

    // Time to study finishes
    pomodoro.update(Message::Tick);

    let last_timer_state = pomodoro.timer_state();

    assert_eq!(last_timer_state.time_to_study, 0);

    assert_eq!(last_timer_state.state(), timer::State::RelaxTime);

    // Time to relax finishes
    pomodoro.update(Message::Tick);

    // Both study and relax timers should be finished
    let last_timer_state = pomodoro.timer_state();

    assert_eq!(last_timer_state.time_to_relax, 0);

    assert_eq!(last_timer_state.time_to_study, 0);

    assert_eq!(last_timer_state.state(), timer::State::Finished);
}
