use bevy::prelude::*;

use crate::{core::item::ItemEnum, ecs::components::Task};

#[derive(Component, Debug)]
#[require(Task)]
pub struct BuyTask {
    pub item: ItemEnum,
    pub qty: usize,
    tried_sellers: Vec<Entity>,
}

impl BuyTask {
    pub fn new(item: ItemEnum, qty: usize) -> Self {
        Self {
            item,
            qty,
            tried_sellers: vec![],
        }
    }

    pub fn tried(&self, seller: &Entity) -> bool {
        self.tried_sellers.contains(seller)
    }

    pub fn add_tried(&mut self, seller: Entity) {
        self.tried_sellers.push(seller);
    }
}
