use bevy::prelude::*;

use crate::ecs::{
    components::Interacting,
    interaction::common::events::InteractionTimedOut,
    knowledge::AgentKnowledge,
    logs::AddLogEntry,
    talk::{
        events::*,
        interaction::{
            components::KnowledgeSharingInteraction,
            events::{SendKnowledgeEvent, ShareKnowledgeFinalizedEvent, StartTalkEvent},
        },
    },
};

// TODO: make this system a generic system for starting every single interaction
pub fn handle_knowlegde_share_requested_system(
    mut query: Query<(
        Entity,
        &mut KnowledgeSharingInteraction,
        &Transform,
        &Interacting,
    )>,
    source_query: Query<(&Transform, &Interacting), With<KnowledgeSharingInteraction>>,
    mut start_talk_writer: EventWriter<StartTalkEvent>,
    mut add_log_writer: EventWriter<AddLogEntry>,
) {
    for (entity, mut knowledge_sharing, entity_transform, entity_interacting) in &mut query {
        let is_source = entity == knowledge_sharing.source;
        if is_source && knowledge_sharing.is_waiting_target() {
            add_log_writer.send(AddLogEntry::new(
                entity,
                format!(
                    "KnowledgeSharingInteraction -> Start with {}",
                    knowledge_sharing.target_name
                )
                .as_str(),
            ));
            // start the interaction for the source
            knowledge_sharing.start();
        } else if is_source {
            // skip if source
        } else if knowledge_sharing.is_waiting_target() {
            // target:
            // check if source is ready (close and dealing with this particular interaction right now)
            if let Ok((source_transform, source_interacting)) =
                source_query.get(knowledge_sharing.source)
            {
                if source_interacting.id != entity_interacting.id {
                    continue;
                }

                if source_transform
                    .translation
                    .distance(entity_transform.translation)
                    <= 50.
                {
                    add_log_writer.send(AddLogEntry::new(
                        entity,
                        format!(
                            "KnowledgeSharingInteraction -> Start with {}",
                            knowledge_sharing.source_name
                        )
                        .as_str(),
                    ));

                    // start the interaction for the target
                    knowledge_sharing.start();
                    start_talk_writer.send(StartTalkEvent {
                        target: knowledge_sharing.target,
                    });
                }
            }
        }
    }
}

pub fn handle_knowlegde_share_started_system(
    target_query: Query<(&KnowledgeSharingInteraction, &AgentKnowledge, &Interacting)>,
    source_query: Query<
        (Entity, &AgentKnowledge),
        (With<KnowledgeSharingInteraction>, With<Interacting>),
    >,
    mut send_knowledge_writer: EventWriter<SendKnowledgeEvent>,
    mut share_knowledge_finalized_writer: EventWriter<ShareKnowledgeFinalizedEvent>,
    mut start_talk_reader: EventReader<StartTalkEvent>,
    mut add_log_writer: EventWriter<AddLogEntry>,
) {
    for event in start_talk_reader.read() {
        if let Ok((knowledge_sharing, target_agent_knowledge, interacting)) =
            &target_query.get(event.target)
        {
            add_log_writer.send(AddLogEntry::new(
                event.target,
                format!("Start talking. ID: {}", interacting.id).as_str(),
            ));

            if let Ok((source_entity, source_agent_knowledge)) =
                source_query.get(knowledge_sharing.source)
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
                        send_knowledge_writer.send(SendKnowledgeEvent {
                            interaction_id: interacting.id,
                            source: source_entity,
                            target: event.target,
                            knowledge_id,
                        });

                        nothing_to_share = false;

                        break; // only ONE will be shared
                    }
                }

                if nothing_to_share {
                    share_knowledge_finalized_writer.send(ShareKnowledgeFinalizedEvent {
                        interaction_id: interacting.id,
                        target: source_entity,
                        success: false,
                        should_trigger_feedback: true,
                    });
                    share_knowledge_finalized_writer.send(ShareKnowledgeFinalizedEvent {
                        interaction_id: interacting.id,
                        target: event.target,
                        success: false,
                        should_trigger_feedback: false,
                    });
                }
            } else {
                // TODO: finish interaction
                println!("No source found: {}", interacting.id);
            }
        }
    }
}

pub fn handle_knowlegde_shared_system(
    mut source_query: Query<
        (
            &KnowledgeSharingInteraction,
            &mut AgentKnowledge,
            &Interacting,
        ),
        (With<KnowledgeSharingInteraction>, With<Interacting>),
    >,
    mut send_knowledge_reader: EventReader<SendKnowledgeEvent>,
    mut share_knowledge_finalized_writer: EventWriter<ShareKnowledgeFinalizedEvent>,
    mut add_log_writer: EventWriter<AddLogEntry>,
) {
    for event in send_knowledge_reader.read() {
        if let Ok((_, source_agent_knowledge, interacting)) =
            &mut source_query.get_mut(event.source)
        {
            add_log_writer.send(AddLogEntry::new(
                event.source,
                format!(
                    "Received requested knowledge. Interaction ID {}",
                    interacting.id
                )
                .as_str(),
            ));

            source_agent_knowledge.add(event.knowledge_id);

            share_knowledge_finalized_writer.send(ShareKnowledgeFinalizedEvent {
                interaction_id: interacting.id,
                target: event.source,
                success: true,
                should_trigger_feedback: true,
            });
        }

        share_knowledge_finalized_writer.send(ShareKnowledgeFinalizedEvent {
            interaction_id: event.interaction_id,
            target: event.target,
            success: true,
            should_trigger_feedback: false,
        });
    }
}

pub fn share_knowledge_finalized_system(
    mut share_knowledge_finalized_reader: EventReader<ShareKnowledgeFinalizedEvent>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut commands: Commands,
) {
    for event in share_knowledge_finalized_reader.read() {
        add_log_writer.send(AddLogEntry::new(
            event.target,
            format!(
                "Finishing Interaction {} - knowledge sharing with {}",
                event.interaction_id,
                if event.success { "SUCCESS" } else { "FAILURE" }
            )
            .as_str(),
        ));

        commands
            .entity(event.target)
            .remove::<(Interacting, KnowledgeSharingInteraction)>();

        if event.should_trigger_feedback {
            if event.success {
                commands.trigger(TalkFinishedWithSuccess {
                    target: event.target,
                });
            } else {
                commands.trigger(TalkFinishedWithFailure {
                    target: event.target,
                });
            }
        }
    }
}

pub fn handle_interaction_timed_out(
    trigger: Trigger<InteractionTimedOut>,
    agent_query: Query<(Entity, &Interacting, &KnowledgeSharingInteraction)>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut commands: Commands,
) {
    for (entity, interacting, _) in &agent_query {
        if trigger.id == interacting.id {
            add_log_writer.send(AddLogEntry::new(
                entity,
                format!(
                    "KnowledgeSharingInteraction -> Interaction {} timed out",
                    interacting.id
                )
                .as_str(),
            ));
            commands
                .entity(entity)
                .remove::<(Interacting, KnowledgeSharingInteraction)>()
                .trigger(TalkFinishedWithFailure { target: entity });
        }
    }
}
