use bevy::{ecs::component::Component, math::Vec3};

pub trait ActionMarker {
    fn should_set_idle_at_completion(&self) -> bool;
    fn set_idle_at_completion(&mut self, value: bool);
}

pub trait DurationAction {
    fn get_resting_duration(&self) -> f32;
    fn progress(&mut self, time: f32);
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
