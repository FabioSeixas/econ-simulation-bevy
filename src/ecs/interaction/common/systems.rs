use bevy::prelude::*;

use crate::ecs::{
    components::*,
    interaction::common::{
        components::AgentInteractionQueue,
        events::{InteractionTimedOut, WaitingInteractionTimedOut},
    },
};

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
