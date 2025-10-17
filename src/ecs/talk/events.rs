use bevy::prelude::*;

use crate::ecs::components::InteractionId;

#[derive(Event, Debug)]
pub struct TalkFinishedWithSuccess {
    pub source: Entity,
}

#[derive(Event, Debug)]
pub struct TalkFinishedWithFailure {
    pub target: Entity,
    pub source: Entity,
    pub interaction_id: InteractionId,
}
