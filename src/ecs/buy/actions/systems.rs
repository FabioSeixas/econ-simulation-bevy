use bevy::prelude::*;

use crate::ecs::buy::actions::components::Buying;
use crate::ecs::buy::actions::components::BuyingFailed;
use crate::ecs::components::*;
use crate::ecs::interaction::common::components::*;
use crate::ecs::interaction::common::events::WaitingInteractionTimedOut;
use crate::ecs::logs::*;
use crate::ecs::sell::actions::components::Selling;
use crate::ecs::trade::components::*;

pub fn handle_buy_action(
    mut query: Query<(Entity, &mut Buying), (Without<Interacting>, Without<WaitingInteraction>)>,
    mut query_seller: Query<&mut AgentInteractionQueue, With<Selling>>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut commands: Commands,
) {
    for (buyer, mut buying) in &mut query {
        if let Ok(mut seller_agent_interaction_queue) = query_seller.get_mut(buying.seller) {
            add_log_writer.send(AddLogEntry::new(
                buyer,
                "Seller found, adding TradeNegotiation to the seller queue",
            ));

            let waiting = WaitingInteraction::new(buyer, buying.seller);
            let interaction_id = waiting.id;

            commands.entity(buyer).insert(waiting);

            let seller_trade_marker = TradeNegotiation {
                role: TradeRole::Seller,
                quantity: buying.qty,
                item: buying.item,
                price: None,
                partner: buyer,
            };

            seller_agent_interaction_queue.add(AgentInteractionItem {
                id: interaction_id,
                kind: AgentInteractionKind::Trade(seller_trade_marker),
            });

            buying.interaction_id = Some(interaction_id);
        } else {
            add_log_writer.send(AddLogEntry::new(buyer, "Seller not found, removing Buying"));

            commands
                .entity(buyer)
                .remove::<Buying>()
                .trigger(BuyingFailed {
                    target: buyer,
                    seller: buying.seller,
                });
        }
    }
}

pub fn handle_waiting_interaction_timed_out(
    trigger: Trigger<WaitingInteractionTimedOut>,
    agent_query: Query<(&WaitingInteraction, &Buying)>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut commands: Commands,
) {
    if let Ok((waiting_interaction, _)) = agent_query.get(trigger.source) {
        if trigger.id == waiting_interaction.id {
            add_log_writer.send(AddLogEntry::new(
                trigger.source,
                format!(
                    "Buying -> WaitingInteraction {} timed out, ending Buying",
                    trigger.id
                )
                .as_str(),
            ));
            commands
                .entity(trigger.source)
                .remove::<(WaitingInteraction, Buying)>()
                .trigger(BuyingFailed {
                    target: trigger.source, // buyer
                    seller: trigger.target, // seller
                });
        }
    }
}
