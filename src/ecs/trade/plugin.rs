use bevy::app::{App, Plugin, Update};

use crate::ecs::trade::events::*;
use crate::ecs::trade::systems::*;
use crate::ecs::trade::tasks::buy::systems::handle_buy_task;
use crate::ecs::trade::actions::buy::systems::handle_buy_action;

pub struct TradePlugin;

impl Plugin for TradePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OfferMade>()
            .add_event::<OfferAgreed>()
            .add_event::<TradeFinalized>()
            .add_systems(
                Update,
                (
                    handle_buy_task,
                    handle_buy_action,
                    seller_makes_offer_system,
                    buyer_evaluates_offer_system,
                    handle_offer_agreed_system,
                    handle_trade_finalized,
                ),
            );
    }
}
