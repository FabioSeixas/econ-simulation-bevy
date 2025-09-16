use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct OfferMade {
    pub target: Entity,
    pub quantity: usize,
    pub price: usize,
}

#[derive(Event, Debug)]
pub struct OfferAgreed {
    pub target: Entity,
}

#[derive(Event, Debug)]
pub struct TradeFinalized {
    pub target: Entity,
}
