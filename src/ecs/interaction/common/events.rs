use bevy::ecs::{entity::Entity, event::Event};

use crate::ecs::components::InteractionId;
use crate::ecs::interaction::common::components::AgentInteractionItem;

#[derive(Event, Debug)]
pub struct InteractionStarted {
    pub target: Entity,
    pub item: AgentInteractionItem,
}

#[derive(Event, Debug)]
pub struct SourceStartInteraction {
    pub target: Entity,
}


#[derive(Event, Debug)]
pub struct InteractionTimedOut {
    // TODO: change to InteractionExit with reason enum (timeout,
    // force quit etc)
    pub id: InteractionId,
}

#[derive(Event, Debug)]
pub struct WaitingInteractionTimedOut {
    // TODO: change to WaitingInteractionExit with reason enum (timeout,
    // force quit etc)
    pub id: InteractionId,
    pub source: Entity,
    pub target: Entity,
}

// The purpose of this event is only to deal with borrow checker:
// In "handle_interaction_starting_for_target_system"
// we need to check two Interacting instances - one from the source
// and other from the target - while beeing able to mutate the one from
// the target. Since this is not allowed, we need this event to mutate elsewhere
#[derive(Event, Debug)]
pub struct TargetIsReadyToStartInteracting {
    pub target: Entity,
}
