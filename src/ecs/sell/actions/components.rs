use std::collections::HashSet;

use bevy::prelude::*;

use crate::ecs::{components::DurationAction, traits::*};

#[derive(Component, Default)]
pub struct Selling {
    resting_duration: f32,
    paused: Paused
}

impl Selling {
    pub fn new() -> Self {
        Self {
            resting_duration: 50.,
            paused: Paused::default()
        }
    }
}

impl Pausable for Selling {
    fn pause(&mut self, reason: PauseReason) {
        self.paused.insert(reason);
    }
    fn resume(&mut self, reason: PauseReason) {
        self.paused.remove(&reason);
    }
    fn is_paused(&self) -> bool {
        self.paused.len() > 0
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
