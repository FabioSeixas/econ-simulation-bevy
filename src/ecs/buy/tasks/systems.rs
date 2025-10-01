use bevy::prelude::*;

use crate::ecs::buy::actions::components::{Buying, BuyingFailed};
use crate::ecs::buy::tasks::components::BuyTask;
use crate::ecs::components::*;
use crate::ecs::knowledge::AgentKnowledge;
use crate::ecs::logs::*;
use crate::ecs::roles::seller::SellerRole;
use crate::ecs::talk::task::components::TalkTask;

pub fn handle_buy_task(
    mut query: Query<
        (Entity, &Transform, &mut BuyTask, &AgentKnowledge),
        (
            Without<Interacting>,
            Without<WaitingInteraction>,
            Without<Buying>,
            Without<Walking>,
        ),
    >,
    mut query_seller: Query<&SellerRole, With<SellerRole>>,
    mut commands: Commands,
    mut add_log_writer: EventWriter<AddLogEntry>,
) {
    for (buyer, buyer_transform, buy_task, buyer_knowledge) in &mut query {
        let mut some_seller_found = false;

        let known_sellers = buyer_knowledge.get_sellers_of(&buy_task.item);

        if known_sellers.len() < 1 {
            add_log_writer.send(AddLogEntry::new(
                buyer,
                "Zero Known Sellers. Buy Task failed. Start TalkTask",
            ));
            commands
                .entity(buyer)
                .insert(TalkTask::new(buy_task.item))
                .remove::<BuyTask>();

            continue;
        }

        for (seller, _) in known_sellers {
            if buy_task.tried(&seller) {
                continue;
            }

            some_seller_found = true;

            if let Ok(seller_role) = query_seller.get_mut(seller.clone()) {
                if buyer_transform.translation.distance(seller_role.location) > 50. {
                    add_log_writer.send(AddLogEntry::new(
                        buyer,
                        "Starting Walking to the seller location",
                    ));
                    let mut walking = Walking::new(seller_role.location);
                    walking.set_idle_at_completion(false);
                    commands.entity(buyer).insert(walking);
                    // buy_task.pause(PauseReason::Walking);
                } else {
                    add_log_writer.send(AddLogEntry::new(buyer, "Start Buying"));
                    // buy_task.pause(PauseReason::Buying);
                    commands.entity(buyer).insert(Buying::new(
                        &buy_task.item,
                        buy_task.qty,
                        seller.clone(),
                    ));
                }
                break;
            }
        }

        if !some_seller_found {
            add_log_writer.send(AddLogEntry::new(buyer, "No seller found. BuyTask failed"));
            add_log_writer.send(AddLogEntry::new(buyer, "Start TalkTask"));
            commands
                .entity(buyer)
                .insert(TalkTask::new(buy_task.item))
                .remove::<BuyTask>();
            // commands
            //     .entity(buyer)
            //     .insert(Walking::new(get_random_vec3()))
            //     .remove::<BuyTask>();
        }
    }
}

pub fn handle_buying_failed(trigger: Trigger<BuyingFailed>, mut query: Query<&mut BuyTask>) {
    if let Ok(mut buy_task) = query.get_mut(trigger.target) {
        // buy_task.resume(PauseReason::Buying);
        buy_task.add_tried(trigger.seller);
    }
}
