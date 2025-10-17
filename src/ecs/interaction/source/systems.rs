use bevy::prelude::*;

use crate::{
    ecs::{
        components::{Interacting, WaitingInteraction},
        interaction::common::{
            components::{AgentInteractionKind, AgentInteractionQueue},
            events::{InteractionStarted, InteractionTimedOut},
        },
        logs::AddLogEntry,
    },
    SourceStartInteraction,
};

pub fn receive_interaction_started_system(
    trigger: Trigger<InteractionStarted>,
    mut query: Query<(
        &WaitingInteraction,
        &mut AgentInteractionQueue,
        Option<&Interacting>,
    )>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut commands: Commands,
) {
    if let Ok((waiting, mut agent_queue, maybe_interacting)) = query.get_mut(trigger.target) {
        add_log_writer.send(AddLogEntry::new(
            trigger.target,
            format!(
                "Received InteractionStarted event Id: {}. WaitingInteraction ID {}",
                trigger.item.id, waiting.id
            )
            .as_str(),
        ));

        if waiting.id != trigger.item.id {
            commands.trigger(InteractionTimedOut {
                id: trigger.item.id,
            });

            return;
        }

        agent_queue.interaction_ready(trigger.item.clone());

        if maybe_interacting.is_none() {
            add_log_writer.send(AddLogEntry::new(
                trigger.target,
                format!(
                    "Triggered SourceStartInteraction event for interaction: {}",
                    trigger.item.id
                )
                .as_str(),
            ));

            commands.trigger(SourceStartInteraction {
                target: trigger.target,
            });
        } else {
            add_log_writer.send(AddLogEntry::new(
                trigger.target,
                format!("Currently interacting {}", maybe_interacting.unwrap().id).as_str(),
            ));
        }
    }
}

pub fn wait_finish_interaction_to_start_new_interaction_as_source_system(
    trigger: Trigger<OnRemove, Interacting>,
    query: Query<(&WaitingInteraction, &AgentInteractionQueue)>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut commands: Commands,
) {
    if let Ok((waiting, agent_queue)) = query.get(trigger.entity()) {
        if let Some(ready_interaction) = agent_queue.get_ready_interaction() {
            if waiting.id != ready_interaction.id {
                panic!("waiting.id != ready_interaction.id should not happen")
            }

            add_log_writer.send(
                AddLogEntry::new(
                    trigger.entity(),
                    format!(
                        "OnRemove<Interacting> -> Triggered SourceStartInteraction event for interaction: {}",
                        ready_interaction.id
                    ).as_str()));

            commands.trigger(SourceStartInteraction {
                target: trigger.entity(),
            });
        }
    }
}

pub fn start_interaction_as_source_system(
    trigger: Trigger<SourceStartInteraction>,
    mut query: Query<(&WaitingInteraction, &mut AgentInteractionQueue)>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut commands: Commands,
) {
    if let Ok((waiting, mut agent_queue)) = query.get_mut(trigger.target) {
        if let Some(ready_interaction) = agent_queue.get_ready_interaction() {
            if waiting.id != ready_interaction.id {
                panic!("waiting.id != ready_interaction.id should not happen")
            }

            add_log_writer.send(AddLogEntry::new(
                trigger.target,
                format!(
                    "Remove WaitingInteraction and starting Interacting. Id: {}",
                    ready_interaction.id
                )
                .as_str(),
            ));

            match &ready_interaction.kind {
                AgentInteractionKind::Ask(sharing) => {
                    commands
                        .entity(sharing.source)
                        .insert((
                            sharing.clone(),
                            Interacting::new_with_id(
                                ready_interaction.id,
                                sharing.source,
                                sharing.target,
                            ),
                        ))
                        .remove::<WaitingInteraction>();
                }
                AgentInteractionKind::Trade(trade_negotiation) => {
                    let interaction = Interacting::new_with_id(
                        ready_interaction.id,
                        trade_negotiation.partner,
                        waiting.target,
                    );

                    commands
                        .entity(trade_negotiation.partner)
                        .insert((
                            interaction,
                            trade_negotiation.clone_for_source(waiting.target),
                        ))
                        .remove::<WaitingInteraction>();
                }
            }

            agent_queue.clean_ready_interaction();
        }
    }
}
