#[derive(Default, Debug, Clone, Copy)]
pub enum View {
    #[default]
    Setup,
    Timers,
}

pub mod setup;
pub mod timers;
