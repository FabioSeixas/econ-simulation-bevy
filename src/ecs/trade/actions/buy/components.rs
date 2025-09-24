use bevy::prelude::*;

use crate::{
    core::item::ItemEnum, ecs::components::Action, ecs::trade::tasks::buy::components::BuyTask,
};

#[derive(Component)]
#[require(Action)]
pub struct Buying {
    pub item: ItemEnum,
    pub qty: usize,
    pub seller: Entity,
    pub interaction_id: Option<usize>,
}

impl Buying {
    pub fn from_buy_task(task: &BuyTask, seller: Entity) -> Self {
        Self {
            qty: task.qty,
            item: task.item,
            seller,
            interaction_id: None,
        }
    }
}
