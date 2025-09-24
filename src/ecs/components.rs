use bevy::{ecs::component::Component, math::Vec3};

pub trait ActionMarker {
    fn should_set_idle_at_completion(&self) -> bool;
    fn set_idle_at_completion(&mut self, value: bool);
}

pub trait DurationActionMarker {
    fn get_resting_duration(&self) -> f32;
    fn progress(&mut self, time: f32);
}

#[derive(Component, Default)]
pub struct Idle;

#[derive(Component, Default)]
pub struct Task;

#[derive(Component, Default)]
pub struct Action;

#[derive(Component, Default, Debug)]
pub struct Interacting;

#[derive(Component, Default, Debug)]
pub struct WaitingInteraction {
    resting_duration: f32,
}

impl WaitingInteraction {
    pub fn new() -> Self {
        Self {
            resting_duration: 5.,
        }
    }
}

impl DurationActionMarker for WaitingInteraction {
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
}

impl ActionMarker for Walking {
    fn set_idle_at_completion(&mut self, value: bool) {
        self.should_set_idle = value;
    }
    fn should_set_idle_at_completion(&self) -> bool {
        self.should_set_idle
    }
}
