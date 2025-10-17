use bevy::prelude::*;

use crate::ecs::{
    components::Interacting,
    interaction::common::{
        components::{AgentInteractionKind, AgentInteractionQueue},
        events::InteractionStarted,
    },
    logs::AddLogEntry,
    trade::components::TradeInteraction,
};

pub fn check_agent_interaction_queue_system(
    mut query: Query<(Entity, &mut AgentInteractionQueue), Without<Interacting>>,
    mut commands: Commands,
    mut add_log_writer: EventWriter<AddLogEntry>,
) {
    for (target_entity, mut agent_interation_queue) in &mut query {
        if !agent_interation_queue.is_empty() {
            let mut maybe_trigger_for_entity: Option<Entity> = None;

            if let Some(interaction_item) = agent_interation_queue.pop_first() {
                match &interaction_item.kind {
                    AgentInteractionKind::Ask(sharing) => {
                        add_log_writer.send(AddLogEntry::new(
                            target_entity,
                            format!(
                                "Received Ask Interaction. source {}. target: {}. Id: {}",
                                sharing.source_name, sharing.target_name, interaction_item.id
                            )
                            .as_str(),
                        ));

                        maybe_trigger_for_entity = Some(sharing.source);
                        commands.entity(target_entity).insert((
                            sharing.clone(),
                            Interacting::new_with_id(
                                interaction_item.id,
                                sharing.source,
                                sharing.target,
                            ),
                        ));
                    }
                    AgentInteractionKind::Trade(trade_negotiation) => {
                        add_log_writer.send(AddLogEntry::new(
                            target_entity,
                            format!(
                                "Received Trade Interaction {}",
                                interaction_item.id
                            )
                            .as_str(),
                        ));

                        // partner here is the source
                        maybe_trigger_for_entity = Some(trade_negotiation.partner);

                        commands.entity(target_entity).insert(TradeInteraction::new(
                            trade_negotiation.clone(),
                            interaction_item.id,
                            trade_negotiation.partner,
                            target_entity,
                        ));
                    }
                };

                if let Some(source_entity) = maybe_trigger_for_entity {
                    commands.trigger(InteractionStarted {
                        item: interaction_item,
                        target: source_entity,
                    });
                }

                // this will start only ONE interaction by frame
                break;
            }
        }
    }
}
