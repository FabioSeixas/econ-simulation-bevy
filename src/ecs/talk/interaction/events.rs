use bevy::prelude::*;

use crate::ecs::knowledge::KnowledgeId;

#[derive(Event, Debug)]
pub struct StartTalkEvent {
    pub target: Entity,
}

#[derive(Event, Debug)]
pub struct SendKnowledgeEvent {
    pub target: Entity,
    pub source: Entity,
    pub knowledge_id: KnowledgeId
}

#[derive(Event, Debug)]
pub struct ShareKnowledgeFinalizedEvent {
    pub target: Entity,
    pub success: bool,
    pub should_trigger_feedback: bool
}
