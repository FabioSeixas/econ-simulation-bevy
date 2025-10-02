use bevy::{ecs::component::Component, math::Vec3};
use rand::random;

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

pub type InteractionId = u32;

#[derive(Component, Debug)]
pub struct Interacting {
    pub id: InteractionId,
    resting_duration: f32,
    timed_out: bool
}

impl Interacting {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_id(id: InteractionId) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn set_timed_out(&mut self) {
        self.timed_out = true;
    }

    pub fn is_timed_out(&self) -> bool {
        self.timed_out
    }
}

impl DurationAction for Interacting {
    fn get_resting_duration(&self) -> f32 {
        self.resting_duration
    }
    fn progress(&mut self, time: f32) {
        self.resting_duration -= time;
    }
}

impl Default for Interacting {
    fn default() -> Self {
        Self {
            id: random(),
            resting_duration: 10.,
            timed_out: false
        }
    }
}

#[derive(Component, Debug)]
pub struct WaitingInteraction {
    resting_duration: f32,
    pub id: InteractionId,
}

impl Default for WaitingInteraction {
    fn default() -> Self {
        Self {
            id: random(),
            resting_duration: 5.,
        }
    }
}

impl WaitingInteraction {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with(resting_duration: f32) -> Self {
        Self {
            resting_duration,
            ..Default::default()
        }
    }
}

impl DurationAction for WaitingInteraction {
    fn get_resting_duration(&self) -> f32 {
        self.resting_duration
    }
    fn progress(&mut self, time: f32) {
        self.resting_duration -= time;
    }
}

#[derive(Component, Debug)]
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

    pub fn new_without_idle(destination: Vec3) -> Self {
        Self {
            destination,
            should_set_idle: false,
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
