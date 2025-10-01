use bevy::prelude::*;

use crate::ecs::trade::events::*;
use crate::ecs::trade::systems::*;
use crate::GameState;

pub struct TradePlugin;

impl Plugin for TradePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OfferMade>()
            .add_event::<OfferAgreed>()
            .add_event::<TradeFinalized>()
            .add_systems(
                Update,
                (
                    seller_makes_offer_system,
                    buyer_evaluates_offer_system,
                    handle_offer_agreed_system,
                    handle_trade_finalized,
                )
                    .run_if(in_state(GameState::Running)),
            );
    }
}
