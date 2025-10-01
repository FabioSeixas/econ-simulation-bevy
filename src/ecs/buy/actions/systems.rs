use bevy::prelude::*;

use crate::ecs::buy::actions::components::Buying;
use crate::ecs::buy::actions::components::BuyingFailed;
use crate::ecs::components::*;
use crate::ecs::interaction::*;
use crate::ecs::logs::*;
use crate::ecs::sell::actions::components::Selling;
use crate::ecs::trade::components::*;

pub fn handle_buy_action(
    mut query: Query<
        (
            Entity,
            &mut Buying,
            Option<&mut WaitingInteraction>,
        ),
        Without<Interacting>, // (With<Buying>, Without<Idle>, Without<Interacting>),
    >,
    mut query_seller: Query<&mut AgentInteractionQueue, With<Selling>>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (buyer, mut buying, waiting_interaction) in &mut query {
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
                    commands
                        .entity(buyer)
                        .remove::<(WaitingInteraction, Buying)>()
                        .trigger(BuyingFailed {
                            target: buyer,
                            seller: buying.seller,
                        });
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

// pub fn handle_waiting_interaction_while_buying(
//     mut query: Query<
//         (Entity, &Buying, &mut BuyTask, &mut WaitingInteraction),
//         Without<Interacting>,
//     >,
//     mut query_seller: Query<&mut AgentInteractionQueue, With<Selling>>,
//     mut add_log_writer: EventWriter<AddLogEntry>,
//     mut commands: Commands,
//     time: Res<Time>,
// ) {
//     for (buyer, buying, mut buy_task, mut waiting) in &mut query {
//         if waiting.get_resting_duration() > 0. {
//             waiting.progress(time.delta_secs());
//         } else {
//             if let Ok(mut seller_interaction_queue) = query_seller.get_mut(buying.seller) {
//                 add_log_writer.send(AddLogEntry::new(
//                     buyer,
//                     "WaitingInteraction timed out, ending Buying",
//                 ));
//                 if buying.interaction_id.is_none() {
//                     panic!("handle_buy_action, buying.interaction_id must be Some")
//                 }
//                 seller_interaction_queue.rm_id(buying.interaction_id.unwrap());
//                 buy_task.add_tried(buying.seller);
//                 commands
//                     .entity(buyer)
//                     .remove::<(WaitingInteraction, Buying)>();
//             }
//         }
//     }
// }
