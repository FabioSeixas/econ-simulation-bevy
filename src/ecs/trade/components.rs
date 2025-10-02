use bevy::prelude::*;

use crate::{
    core::item::ItemEnum,
    ecs::components::{Interacting, InteractionId},
};

#[derive(Component, Clone, Copy, Debug)]
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
    pub fn new(trade: TradeNegotiation, interaction_id: InteractionId, source: Entity, target: Entity) -> Self {
        Self {
            interacting: Interacting::new_with_id(interaction_id, source, target),
            trade,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TradeRole {
    Buyer,
    Seller,
}
