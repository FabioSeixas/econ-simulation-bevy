use bevy::prelude::*;

use crate::{core::item::ItemEnum, ecs::components::*, ecs::utils::get_random_vec3};

#[derive(Component)]
#[require(Task)]
pub struct ConsumeTask {
    pub location: Vec3,
    pub item: ItemEnum,
    pub qty: usize,
}

impl ConsumeTask {
    pub fn new(item: ItemEnum, qty: usize) -> Self {
        Self {
            location: get_random_vec3(),
            item,
            qty,
        }
    }
}
