use bevy::prelude::*;

use crate::ecs::{components::InteractionId, knowledge::KnowledgeId};

#[derive(Event, Debug)]
pub struct StartTalkEvent {
    pub target: Entity,
    pub source: Entity,
    pub interaction_id: InteractionId
}

#[derive(Event, Debug)]
pub struct SendKnowledgeEvent {
    pub interaction_id: InteractionId,
    pub target: Entity,
    pub source: Entity,
    pub knowledge_id: KnowledgeId
}

#[derive(Event, Debug)]
pub struct ShareKnowledgeFinalizedEvent {
    pub interaction_id: InteractionId,
    pub target: Entity,
    pub source: Entity,
    pub success: bool,
}
