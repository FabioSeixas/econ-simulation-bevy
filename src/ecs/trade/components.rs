use bevy::prelude::*;

use crate::{
    core::item::ItemEnum,
    ecs::components::{DurationActionMarker, Interacting},
};

#[derive(Component, Debug)]
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

#[derive(Component)]
pub struct Buying {
    pub item: ItemEnum,
    pub qty: usize,
    pub seller: Entity,
    pub interaction_id: Option<usize>
}

impl Buying {
    pub fn from_buy_task(task: &BuyTask, seller: Entity) -> Self {
        Self {
            qty: task.qty,
            item: task.item,
            seller,
            interaction_id: None
        }
    }
}

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

impl DurationActionMarker for Selling {
    fn get_resting_duration(&self) -> f32 {
        self.resting_duration
    }
    fn progress(&mut self, time: f32) {
        self.resting_duration -= time;
    }
}

#[derive(Component, Clone, Debug)]
pub struct TradeNegotiation {
    pub partner: Entity,
    pub role: TradeRole,
    pub item: ItemEnum,
    pub quantity: usize,
    pub price: Option<usize>,
}

#[derive(Bundle, Debug)]
pub struct TradeInteraction {
    interacting: Interacting,
    pub trade: TradeNegotiation,
}

impl TradeInteraction {
    pub fn new(trade: TradeNegotiation) -> Self {
        Self {
            interacting: Interacting,
            trade,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TradeRole {
    Buyer,
    Seller,
}
