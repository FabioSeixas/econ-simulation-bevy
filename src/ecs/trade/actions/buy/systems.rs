use bevy::prelude::*;

use crate::ecs::components::*;
use crate::ecs::interaction::*;
use crate::ecs::logs::*;
use crate::ecs::trade::actions::sell::components::Selling;
use crate::ecs::trade::components::*;
use crate::ecs::trade::tasks::buy::components::BuyTask;
use crate::ecs::trade::actions::buy::components::Buying;

pub fn handle_buy_action(
    mut query: Query<
        (
            Entity,
            &mut Buying,
            &mut BuyTask,
            Option<&mut WaitingInteraction>,
        ),
        (With<Buying>, Without<Idle>, Without<Interacting>),
    >,
    mut query_seller: Query<&mut AgentInteractionQueue, With<Selling>>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (buyer, mut buying, mut buy_task, waiting_interaction) in &mut query {
        if let Some(mut waiting) = waiting_interaction {
            if waiting.get_resting_duration() > 0. {
                waiting.progress(time.delta_secs());
            } else {
                if let Ok(mut seller_interaction_queue) = query_seller.get_mut(buying.seller) {
                    add_log_writer.send(AddLogEntry::new(
                        buyer,
                        "WaitingInteraction timed out, ending Buying",
                    ));
                    if buying.interaction_id.is_none() {
                        panic!("handle_buy_action, buying.interaction_id must be Some")
                    }
                    seller_interaction_queue.rm_id(buying.interaction_id.unwrap());
                    buy_task.add_tried(buying.seller);
                    commands
                        .entity(buyer)
                        .remove::<(WaitingInteraction, Buying)>();
                }
            }
        } else if let Ok(mut seller_agent_interaction_queue) = query_seller.get_mut(buying.seller) {
            add_log_writer.send(AddLogEntry::new(
                buyer,
                "Seller found, adding TradeNegotiation to the seller queue",
            ));
            let buyer_trade_marker = TradeNegotiation {
                role: TradeRole::Buyer,
                quantity: buying.qty,
                item: buying.item,
                price: None,
                partner: buying.seller.clone(),
            };
            commands
                .entity(buyer)
                .insert((buyer_trade_marker, WaitingInteraction::new()));

            let seller_trade_marker = TradeNegotiation {
                role: TradeRole::Seller,
                quantity: buying.qty,
                item: buying.item,
                price: None,
                partner: buyer,
            };

            let id = seller_agent_interaction_queue
                .add(AgentInteractionKind::Trade(seller_trade_marker));

            buying.interaction_id = Some(id);
        } else {
            add_log_writer.send(AddLogEntry::new(buyer, "Seller not found, removing Buying"));
            buy_task.add_tried(buying.seller);
            commands.entity(buyer).remove::<Buying>();
        }
    }
}
