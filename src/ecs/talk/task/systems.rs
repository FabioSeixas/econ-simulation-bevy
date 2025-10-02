use bevy::prelude::*;

use crate::ecs::agent::Agent;
use crate::ecs::components::*;
use crate::ecs::interaction::*;
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
                let waiting = WaitingInteraction::new_with(10.);
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

pub fn handle_waiting_while_talk_task(
    mut source_agent_query: Query<(Entity, &TalkTask, &mut WaitingInteraction)>,
    mut target_agent_query: Query<&mut AgentInteractionQueue>,
    time: Res<Time>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut commands: Commands,
) {
    for (source_entity, task, mut waiting) in &mut source_agent_query {
        if waiting.get_resting_duration() > 0. {
            waiting.progress(time.delta_secs());
        } else {
            add_log_writer.send(AddLogEntry::new(
                source_entity,
                format!("WaitingInteraction {} timed out.", waiting.id).as_str(),
            ));

            if let Some((interaction_id, target_entity, _)) = task.current_interaction.as_ref() {
                if let Ok(mut target_interaction_queue) =
                    target_agent_query.get_mut(target_entity.clone())
                {
                    target_interaction_queue.rm_id(interaction_id.clone());
                }
            }

            commands
                .entity(source_entity)
                .remove::<WaitingInteraction>()
                .trigger(TalkFinishedWithFailure {
                    target: source_entity,
                });
        }
    }
}

pub fn handle_get_close_to_target_while_talk_task(
    source_agent_query: Query<(Entity, &Transform, &TalkTask), Without<Walking>>,
    target_agent_query: Query<(Entity, &Transform), With<Agent>>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut commands: Commands,
) {
    for (source_entity, source_transform, talk_task) in &source_agent_query {
        if let Some((_, target_entity, target_name)) = talk_task.current_interaction.as_ref() {
            if let Ok((_, target_transform)) = target_agent_query.get(target_entity.clone()) {
                if source_transform
                    .translation
                    .distance(target_transform.translation)
                    > 50.
                {
                    add_log_writer.send(AddLogEntry::new(
                        source_entity,
                        format!("TalkTask -> Walking to target {}", target_name).as_str(),
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
        // println!(
        //     "handle_talk_failure: {} at {:?} - {:?}",
        //     entity,
        //     SystemTime::now().duration_since(UNIX_EPOCH).ok().unwrap(),
        //     task
        // );

        add_log_writer.send(AddLogEntry::new(entity, "handle_talk_failure"));

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

        // let (_, partner, partner_name) = task
        //     .current_interaction
        //     .take()
        //     .expect("current_interaction must be Some");

        // add_log_writer.send(AddLogEntry::new(
        //     entity,
        //     format!(
        //         "TalkTask -> failed with {}, will try with another Agent",
        //         partner_name
        //     )
        //     .as_str(),
        // ));
        //
        // task.tried.push(partner);
    }
}
