use std::collections::HashSet;

use bevy::prelude::*;

use crate::{
    core::item::ItemEnum,
    ecs::{traits::*, utils::get_random_vec3},
};

#[derive(Component)]
pub struct ConsumeTask {
    pub location: Vec3,
    pub item: ItemEnum,
    pub qty: usize,
    paused: HashSet<PauseReason>,
}

impl ConsumeTask {
    pub fn new(item: ItemEnum, qty: usize) -> Self {
        Self {
            location: get_random_vec3(),
            item,
            qty,
            paused: HashSet::new(),
        }
    }
}

impl Pausable for ConsumeTask {
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
