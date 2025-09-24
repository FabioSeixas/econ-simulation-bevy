use bevy::prelude::*;

use crate::ecs::components::DurationAction;

#[derive(Component, Default)]
pub struct Selling {
    resting_duration: f32,
}

impl Selling {
    pub fn new() -> Self {
        Self {
            resting_duration: 50.,
        }
    }
}

impl DurationAction for Selling {
    fn get_resting_duration(&self) -> f32 {
        self.resting_duration
    }
    fn progress(&mut self, time: f32) {
        self.resting_duration -= time;
    }
}
