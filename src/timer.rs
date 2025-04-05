#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Settings {
    pub time_to_study: u64,
    pub time_to_relax: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    StudyTime,
    RelaxTime,
    Finished,
}

impl Settings {
    pub fn state(&self) -> State {
        if self.time_to_study > 0 {
            State::StudyTime
        } else if self.time_to_relax > 0 {
            State::RelaxTime
        } else {
            State::Finished
        }
    }
}
