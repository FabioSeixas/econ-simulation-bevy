use bevy::prelude::{Entity, Event};

use crate::core::item::ItemEnum;

#[derive(Event, Debug, Clone)]
pub struct AgentInteraction {
    pub source: Entity,
    pub target: Entity,
    pub trade: Option<Trade>,
}

impl AgentInteraction {
    pub fn new(source: Entity, target: Entity) -> Self {
        Self {
            source,
            target,
            trade: None,
        }
    }

    pub fn new_with_trade(source: Entity, target: Entity, trade: Option<Trade>) -> Self {
        Self {
            source,
            target,
            trade,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TradeStatus {
    NEGOTIATION,
    DONE,
}

#[derive(Clone, Debug)]
pub struct Trade {
    pub item: ItemEnum,
    pub qty: usize,
    pub price: Option<usize>,
    status: TradeStatus,
}

impl Trade {
    pub fn new(item: &ItemEnum, qty: usize) -> Self {
        Self {
            item: item.clone(),
            qty,
            price: None,
            status: TradeStatus::NEGOTIATION,
        }
    }

    pub fn buyer_accepted(&mut self) {
        self.status = TradeStatus::DONE
    }

    pub fn get_status(&self) -> TradeStatus {
        self.status
    }
}
