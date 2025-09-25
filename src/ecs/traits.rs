use std::collections::HashSet;

pub trait Pausable {
    fn pause(&mut self, reason: PauseReason);
    fn resume(&mut self, reason: PauseReason);
    fn is_paused(&self) -> bool;
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum PauseReason {
    Interacting,
    Walking,
    Consuming,
}

pub type Paused = HashSet<PauseReason>;
