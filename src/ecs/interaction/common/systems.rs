use bevy::prelude::*;

use crate::ecs::{
    components::*,
    interaction::common::{
        components::AgentInteractionQueue,
        events::{
            InteractionTimedOut, TargetIsReadyToStartInteracting, WaitingInteractionTimedOut,
        },
    },
    logs::AddLogEntry,
};

// This system must be generic for starting every single interaction (source)
pub fn handle_interaction_starting_for_source_system(
    mut query: Query<(Entity, &mut Interacting)>,
    mut add_log_writer: EventWriter<AddLogEntry>,
) {
    for (entity, mut entity_interacting) in &mut query {
        if entity != entity_interacting.source {
            continue;
        }

        if entity_interacting.is_waiting() {
            add_log_writer.send(AddLogEntry::new(
                entity,
                format!(
                    "Interaction {} -> Set Ready as Source",
                    entity_interacting.id
                )
                .as_str(),
            ));
            // start the interaction for the source
            entity_interacting.set_ready();
        }
    }
}

// This system must be generic for starting every single interaction (target)
pub fn handle_interaction_starting_for_target_system(
    query: Query<(Entity, &Transform, &Interacting)>,
    source_query: Query<(&Transform, &Interacting)>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut command: Commands,
) {
    for (entity, entity_transform, entity_interacting) in &query {
        if entity != entity_interacting.target {
            continue;
        }

        // With the rules here, it means the source get ready first
        // and the target get ready later.

        if entity_interacting.is_waiting() {
            // check if source is ready:
            // 1. close enough
            // 2. dealing with this particular interaction right now (by interaction id)
            // 3. is already ready to start interacting
            if let Ok((source_transform, source_interacting)) =
                source_query.get(entity_interacting.source)
            {
                if source_interacting.id != entity_interacting.id {
                    continue;
                }

                // TODO: confirm this is safe and work as expected
                if source_interacting.is_waiting() {
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
                            "Interaction {} -> Set Ready as Target",
                            entity_interacting.id
                        )
                        .as_str(),
                    ));

                    command.trigger(TargetIsReadyToStartInteracting { target: entity });
                }
            }
        }
    }
}

pub fn target_is_ready_to_start_interacting(
    trigger: Trigger<TargetIsReadyToStartInteracting>,
    mut query: Query<&mut Interacting>,
) {
    if let Ok(mut target_interacting) = query.get_mut(trigger.target) {
        // start the interaction for the target
        target_interacting.set_ready();
    }
}

pub fn interaction_timeout_system(
    mut query: Query<&mut Interacting>,
    mut command: Commands,
    time: Res<Time>,
) {
    for mut interacting in &mut query {
        if interacting.get_resting_duration() > 0. {
            interacting.progress(time.delta_secs());
        } else if interacting.is_timed_out() {
            // nothing
        } else {
            interacting.set_timed_out();
            command.trigger(InteractionTimedOut { id: interacting.id });
        }
    }
}

pub fn waiting_interaction_timeout_system(
    mut query: Query<&mut WaitingInteraction>,
    mut command: Commands,
    time: Res<Time>,
) {
    for mut waiting in &mut query {
        if waiting.get_resting_duration() > 0. {
            waiting.progress(time.delta_secs());
        } else if waiting.is_timed_out() {
            // nothing
        } else {
            waiting.set_timed_out();
            command.trigger(WaitingInteractionTimedOut {
                id: waiting.id,
                source: waiting.source,
                target: waiting.target,
            });
        }
    }
}

pub fn remove_timed_out_interaction_from_agent_queue(
    trigger: Trigger<InteractionTimedOut>,
    mut query: Query<(&mut AgentInteractionQueue, &Interacting)>,
) {
    for (mut agent_interation_queue, _) in &mut query {
        if !agent_interation_queue.is_empty() {
            agent_interation_queue.rm_id(trigger.id);
        }
    }
}

pub fn remove_timed_out_waiting_interaction_from_agent_queue(
    trigger: Trigger<WaitingInteractionTimedOut>,
    mut query: Query<&mut AgentInteractionQueue>,
) {
    if let Ok(mut agent_interation_queue) = query.get_mut(trigger.source) {
        if !agent_interation_queue.is_empty() {
            agent_interation_queue.rm_id(trigger.id);
        }
    }
    if let Ok(mut agent_interation_queue) = query.get_mut(trigger.target) {
        if !agent_interation_queue.is_empty() {
            agent_interation_queue.rm_id(trigger.id);
        }
    }
}
