use bevy::prelude::*;

use crate::{
    core::item::ItemEnum,
    ecs::components::{DurationAction, Interacting},
};

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
