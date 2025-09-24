use bevy::{ecs::component::Component, math::Vec3};

pub trait ActionMarker {
    fn should_set_idle_at_completion(&self) -> bool;
    fn set_idle_at_completion(&mut self, value: bool);
}

pub trait DurationAction {
    fn get_resting_duration(&self) -> f32;
    fn progress(&mut self, time: f32);
}

#[derive(Component, Default)]
pub struct Idle;

#[derive(Component, Default, PartialEq, Eq)]
pub enum AgentState {
    #[default]
    Idle,
    Tasking,
    Acting,
    Interacting(InteractingStep),
}

impl AgentState {
    pub fn interaction_happening() -> Self {
        AgentState::Interacting(InteractingStep::Happening)
    }

    pub fn is_idle(&self) -> bool {
        *self == AgentState::Idle
    }

    pub fn is_acting(&self) -> bool {
        *self == AgentState::Acting
    }

    pub fn is_actively_interacting(&self) -> bool {
        match self {
            Self::Interacting(v) => match v {
                InteractingStep::Happening => true,
                _ => false,
            },
            _ => false,
        }
    }
}

#[derive(Component, Default, PartialEq, Eq)]
pub enum InteractingStep {
    #[default]
    Waiting,
    Happening,
}

#[derive(Component, Default, Debug)]
pub struct Interacting;

#[derive(Component, Debug)]
#[require(AgentState(|| AgentState::Acting))]
pub struct Walking {
    pub destination: Vec3,
    should_set_idle: bool,
}

impl Walking {
    pub fn new(destination: Vec3) -> Self {
        Self {
            destination,
            should_set_idle: true,
        }
    }
}

impl ActionMarker for Walking {
    fn set_idle_at_completion(&mut self, value: bool) {
        self.should_set_idle = value;
    }
    fn should_set_idle_at_completion(&self) -> bool {
        self.should_set_idle
    }
}
