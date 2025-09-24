use bevy::prelude::*;

use crate::ecs::{
    components::{Idle, Interacting, WaitingInteraction},
    knowledge::AgentKnowledge,
    logs::AddLogEntry,
    talk::{
        interaction::{
            components::KnowledgeSharingInteraction,
            events::{SendKnowledgeEvent, ShareKnowledgeFinalizedEvent},
        }, task::components::TalkTask,
    },
};

pub fn handle_knowlegde_share_request_system(
    target_query: Query<
        (
            Entity,
            &KnowledgeSharingInteraction,
            &AgentKnowledge,
            Option<&TalkTask>,
        ),
        (
            Added<KnowledgeSharingInteraction>,
            With<Interacting>,
            Without<WaitingInteraction>,
        ),
    >,
    source_query: Query<
        (Entity, &AgentKnowledge),
        (With<KnowledgeSharingInteraction>, With<Interacting>),
    >,
    mut share_knowledge_writer: EventWriter<SendKnowledgeEvent>,
    mut share_knowledge_finalized_writer: EventWriter<ShareKnowledgeFinalizedEvent>,
    mut add_log_writer: EventWriter<AddLogEntry>,
) {
    for (target_entity, knowledge_sharing, target_agent_knowledge, task_opt) in &target_query {
        if task_opt.is_some() {
            continue;
        }

        add_log_writer.send(AddLogEntry::new(
            target_entity,
            "arrive handle_knowlegde_share_request_system",
        ));
        if let Ok((source_entity, source_agent_knowledge)) =
            source_query.get(knowledge_sharing.partner)
        {
            let target_known_sellers =
                target_agent_knowledge.get_sellers_of(&knowledge_sharing.seller_of);
            let source_known_sellers: Vec<Entity> = source_agent_knowledge
                .get_sellers_of(&knowledge_sharing.seller_of)
                .iter()
                .map(|(seller, _)| seller.clone())
                .collect();

            let mut nothing_to_share = true;

            for (seller, knowledge_id) in target_known_sellers {
                if !source_known_sellers.contains(&seller) {
                    share_knowledge_writer.send(SendKnowledgeEvent {
                        target: source_entity,
                        knowledge_id,
                    });

                    nothing_to_share = false;

                    break; // only ONE will be shared
                }
            }

            if nothing_to_share {
                share_knowledge_finalized_writer.send(ShareKnowledgeFinalizedEvent {
                    target: source_entity,
                    success: false,
                });
                share_knowledge_finalized_writer.send(ShareKnowledgeFinalizedEvent {
                    target: target_entity,
                    success: false,
                });
            }
        }
    }
}

pub fn handle_knowlegde_share_finalized_system(
    mut target_query: Query<
        (Entity, &KnowledgeSharingInteraction, &mut AgentKnowledge),
        (With<KnowledgeSharingInteraction>, With<Interacting>),
    >,
    mut share_knowledge_reader: EventReader<SendKnowledgeEvent>,
    mut share_knowledge_finalized_writer: EventWriter<ShareKnowledgeFinalizedEvent>,
) {
    for event in share_knowledge_reader.read() {
        if let Ok((target_entity, knowledge_sharing, target_agent_knowledge)) =
            &mut target_query.get_mut(event.target)
        {
            target_agent_knowledge.add(event.knowledge_id);

            share_knowledge_finalized_writer.send(ShareKnowledgeFinalizedEvent {
                target: target_entity.clone(),
                success: true,
            });
            share_knowledge_finalized_writer.send(ShareKnowledgeFinalizedEvent {
                target: knowledge_sharing.partner,
                success: true,
            });
        } else {
            panic!("Target of ShareKnowledgeEvent was not found")
        }
    }
}

pub fn share_knowledge_finalized_system(
    mut share_knowledge_finalized_reader: EventReader<ShareKnowledgeFinalizedEvent>,
    mut agent_query: Query<
        (Entity, Option<&mut TalkTask>),
        (With<KnowledgeSharingInteraction>, With<Interacting>),
    >,
    mut commands: Commands,
) {
    for event in share_knowledge_finalized_reader.read() {
        println!("event: {:?}", event);
        if let Ok((_, task_opt)) = agent_query.get_mut(event.target) {
            if let Some(mut task) = task_opt {
                // source
                if event.success {
                    commands.entity(event.target).insert(Idle).remove::<(
                        Interacting,
                        KnowledgeSharingInteraction,
                        TalkTask
                    )>();
                } else {
                    println!("task: {:?}", task);
                    if task.current_interaction.is_none() {
                        panic!("current_interaction must be Some")
                    }

                    let partner = task.current_interaction.unwrap().1;
                    task.tried.push(partner);
                    task.current_interaction = None;
                    commands
                        .entity(event.target)
                        .remove::<(Interacting, KnowledgeSharingInteraction)>();
                }
            } else {
                // target
                commands.entity(event.target).insert(Idle).remove::<(
                    Interacting,
                    KnowledgeSharingInteraction,
                    TalkTask,
                )>();
            }
        }
    }
}
