use bevy::prelude::*;

use crate::{core::item::ItemEnum, ecs::components::DurationActionMarker};

#[derive(Component)]
pub struct Consuming {
    resting_duration: f32,
    pub item: ItemEnum,
    pub qty: usize,
}

impl Consuming {
    pub fn new(item: ItemEnum, qty: usize) -> Self {
        Self {
            item,
            qty,
            resting_duration: 5. * (qty as f32),
        }
    }
}

impl DurationActionMarker for Consuming {
    fn get_resting_duration(&self) -> f32 {
        self.resting_duration
    }
    fn progress(&mut self, time: f32) {
        self.resting_duration -= time;
    }
}
