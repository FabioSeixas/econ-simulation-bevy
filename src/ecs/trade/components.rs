use bevy::prelude::*;

use crate::{core::item::ItemEnum, ecs::components::Interacting};

#[derive(Component)]
pub struct Buying {
    pub item: ItemEnum,
    pub qty: usize
}

#[derive(Component, Default)]
pub struct Selling;

#[derive(Component, Clone, Debug)]
pub struct TradeNegotiation {
    pub partner: Entity,
    pub role: TradeRole,
    pub item: ItemEnum,
    pub quantity: usize,
    pub price: Option<usize>,
    // pub status: TradeStatus,
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

// #[derive(Clone, Debug)]
// pub enum TradeStatus {
//     Initiated,
//     OfferMade,
//     Agreed,
//     Finalized,
//     Failed,
// }

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TradeRole {
    Buyer,
    Seller,
}
