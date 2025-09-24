use bevy::prelude::*;

use crate::ecs::agent::Agent;
use crate::ecs::components::*;
use crate::ecs::interaction::*;
use crate::ecs::logs::*;
use crate::ecs::talk::interaction::components::KnowledgeSharingInteraction;
use crate::ecs::talk::task::components::TalkTask;

pub fn handle_talk_task_system(
    mut source_agent_query: Query<
        (Entity, &Transform, &mut TalkTask),
        (
            With<TalkTask>,
            Without<Interacting>,
            Without<WaitingInteraction>,
        ),
    >,
    mut target_agent_query: Query<(Entity, &Transform, &mut AgentInteractionQueue), With<Agent>>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut commands: Commands,
) {
    for (source_entity, source_transform, mut obtain_knowledge_task) in &mut source_agent_query {
        if obtain_knowledge_task.current_interaction.is_some() {
            continue;
        }

        let mut best: Option<(Entity, f32)> = None;
        for (entity, target_transform, _) in &target_agent_query {
            if entity.eq(&source_entity) {
                continue;
            }
            if obtain_knowledge_task.tried.contains(&entity) {
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
            if let Ok((_, _, mut agent_interation_queue)) =
                target_agent_query.get_mut(closest_entity)
            {
                add_log_writer.send(AddLogEntry::new(
                    source_entity,
                    format!("Sent Ask Interaction request for {}", closest_entity).as_str(),
                ));

                obtain_knowledge_task.current_interaction = Some((
                    agent_interation_queue.add(AgentInteractionKind::Ask(
                        KnowledgeSharingInteraction {
                            seller_of: obtain_knowledge_task.content.seller_of,
                            partner: source_entity,
                        },
                    )),
                    closest_entity,
                ));

                commands
                    .entity(source_entity)
                    .insert(WaitingInteraction::new());
            }
        } else {
            // TODO
            info!("no agents found");
        }
    }
}

pub fn start_talk_interaction_system(
    mut source_agent_query: Query<
        (Entity, &Transform, &mut TalkTask),
        (
            With<TalkTask>,
            With<WaitingInteraction>,
            Without<Walking>,
            Without<Interacting>,
        ),
    >,
    mut target_agent_query: Query<(Entity, &Transform, &mut AgentInteractionQueue), With<Agent>>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut commands: Commands,
) {
    for (source_entity, source_transform, obtain_knowledge_task) in &mut source_agent_query {
        if let Some((_, target_entity)) = obtain_knowledge_task.current_interaction {
            if let Ok((_, target_transform, _)) = target_agent_query.get_mut(target_entity) {
                if source_transform
                    .translation
                    .distance(target_transform.translation)
                    > 50.
                {
                    add_log_writer.send(AddLogEntry::new(
                        source_entity,
                        format!("Walking to for {}", target_entity).as_str(),
                    ));
                    let mut walking = Walking::new(
                        (target_transform.translation - source_transform.translation).normalize(),
                    );
                    walking.set_idle_at_completion(false);
                    commands.entity(source_entity).insert(walking);
                }
            }
        }
    }
}
