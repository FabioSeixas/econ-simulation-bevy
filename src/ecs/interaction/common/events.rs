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
