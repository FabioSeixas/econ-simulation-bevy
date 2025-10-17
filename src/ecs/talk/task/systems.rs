use bevy::prelude::*;

use crate::ecs::agent::Agent;
use crate::ecs::components::*;
use crate::ecs::interaction::common::components::{AgentInteractionItem, AgentInteractionKind, AgentInteractionQueue};
use crate::ecs::interaction::common::events::WaitingInteractionTimedOut;
use crate::ecs::logs::*;
use crate::ecs::talk::events::*;
use crate::ecs::talk::interaction::components::KnowledgeSharingInteraction;
use crate::ecs::talk::task::components::TalkTask;

pub fn handle_added_talk_task(
    mut source_agent_query: Query<(Entity, &Transform, &Name, &mut TalkTask), Without<Interacting>>,
    mut target_agent_query: Query<(Entity, &Transform, &Name, &mut AgentInteractionQueue)>, // maybe without<Interaction>
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut commands: Commands,
) {
    for (source_entity, source_transform, source_name, mut talk_task) in &mut source_agent_query {
        if talk_task.current_interaction.is_some() {
            continue;
        }

        add_log_writer.send(AddLogEntry::new(
            source_entity,
            format!("TalkTask -> searching an Agent to ask").as_str(),
        ));

        if talk_task.tried.len() > 3 {
            // although using TalkFinishedWithSuccess here,
            // this is a failure
            commands.trigger(TalkFinishedWithSuccess {
                target: source_entity
            });
        }

        let mut best: Option<(Entity, f32)> = None;
        for (entity, target_transform, _, _) in &target_agent_query {
            if entity.eq(&source_entity) {
                continue;
            }
            if talk_task.tried.contains(&entity) {
                continue;
            }

            let d2 = source_transform
                .translation
                .distance_squared(target_transform.translation);

            // TODO: set a maximum acceptable distance
            match best {
                None => best = Some((entity, d2)),
                Some((_, best_d2)) => {
                    if d2 < best_d2 {
                        best = Some((entity, d2))
                    }
                }
            }
        }

        if let Some((closest_entity, _)) = best {
            if let Ok((_, _, name, mut agent_interation_queue)) =
                target_agent_query.get_mut(closest_entity)
            {
                let waiting =
                    WaitingInteraction::new_with_duration(source_entity, closest_entity, 10.);
                let interaction_id = waiting.id;

                add_log_writer.send(AddLogEntry::new(
                    source_entity,
                    format!(
                        "Sent Ask Interaction request for {}. ID: {}",
                        name, interaction_id
                    )
                    .as_str(),
                ));

                agent_interation_queue.add(AgentInteractionItem {
                    id: interaction_id,
                    kind: AgentInteractionKind::Ask(KnowledgeSharingInteraction::new(
                        talk_task.seller_of,
                        source_entity,       // source
                        closest_entity,      // target
                        source_name.clone(), // source name
                        name.clone(),        // target name
                    )),
                });

                talk_task.current_interaction =
                    Some((interaction_id, closest_entity, name.clone()));

                commands.entity(source_entity).insert(waiting);
            }
        } else {
            // TODO
            info!("no agents found");
        }
    }
}

pub fn handle_waiting_interaction_timed_out(
    trigger: Trigger<WaitingInteractionTimedOut>,
    agent_query: Query<(&WaitingInteraction, &TalkTask)>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut commands: Commands,
) {
    if let Ok((waiting_interaction, _)) = agent_query.get(trigger.source) {
        if trigger.id == waiting_interaction.id {
            add_log_writer.send(AddLogEntry::new(
                trigger.source,
                format!(
                    "TalkTask -> WaitingInteraction {} timed out",
                    waiting_interaction.id
                )
                .as_str(),
            ));

            commands
                .entity(trigger.source)
                .remove::<WaitingInteraction>()
                .trigger(TalkFinishedWithFailure {
                    target: trigger.source,
                });
        }
    }
}

pub fn handle_get_close_to_target_while_talk_task(
    source_agent_query: Query<
        (Entity, &Transform, &TalkTask, Option<&Interacting>),
        Without<Walking>,
    >,
    target_agent_query: Query<(Entity, &Transform), With<Agent>>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut commands: Commands,
) {
    for (source_entity, source_transform, talk_task, maybe_interacting) in &source_agent_query {
        if let Some((current_interaction_id, target_entity, target_name)) =
            talk_task.current_interaction.as_ref()
        {
            if let Some(interacting) = maybe_interacting {
                if interacting.id != *current_interaction_id {
                    continue;
                }
            }

            if let Ok((_, target_transform)) = target_agent_query.get(target_entity.clone()) {
                if source_transform
                    .translation
                    .distance(target_transform.translation)
                    > 50.
                {
                    add_log_writer.send(AddLogEntry::new(
                        source_entity,
                        format!(
                            "TalkTask -> Walking to target {}. Interaction ID {}",
                            target_name, current_interaction_id
                        )
                        .as_str(),
                    ));
                    commands
                        .entity(source_entity)
                        .insert(Walking::new_without_idle(target_transform.translation));
                }
            }
        }
    }
}

pub fn handle_talk_success(
    trigger: Trigger<TalkFinishedWithSuccess>,
    agent_query: Query<&TalkTask>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut commands: Commands,
) {
    let entity = trigger.target;
    if let Ok(_) = agent_query.get(entity) {
        add_log_writer.send(AddLogEntry::new(entity, "TalkTask -> Back to Idle"));
        commands.entity(entity).insert(Idle).remove::<TalkTask>();
    }
}

pub fn handle_talk_failure(
    trigger: Trigger<TalkFinishedWithFailure>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut agent_query: Query<&mut TalkTask>,
) {
    let entity = trigger.target;
    if let Ok(mut task) = agent_query.get_mut(entity) {
        if let Some((id, partner, partner_name)) = task.current_interaction.take() {
            add_log_writer.send(AddLogEntry::new(
                entity,
                format!(
                    "TalkTask -> interaction {} failed with {}, will try with another Agent",
                    id, partner_name
                )
                .as_str(),
            ));

            task.tried.insert(partner);
        }
    }
}
