use bevy::prelude::*;

use crate::{core::item::ItemEnum, ecs::components::InteractionId};

#[derive(Component)]
pub struct Buying {
    pub item: ItemEnum,
    pub qty: usize,
    pub seller: Entity,
    pub interaction_id: Option<InteractionId>,
}

impl Buying {
    pub fn new(item: &ItemEnum, qty: usize, seller: Entity) -> Self {
        Self {
            qty: qty,
            item: item.clone(),
            seller,
            interaction_id: None,
        }
    }
}

#[derive(Event)]
pub struct BuyingSucceeded {
    pub target: Entity,
}

#[derive(Event)]
pub struct BuyingFailed {
    pub target: Entity,
    pub seller: Entity,
}
