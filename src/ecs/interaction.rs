use bevy::ecs::{component::Component, entity::Entity};

use crate::{core::item::ItemEnum, ecs::trade::components::TradeNegotiation};

#[derive(Debug)]
pub struct AgentInteractionEvent {
    pub id: usize,
    pub kind: AgentInteractionKind,
}

#[derive(Debug)]
pub enum AgentInteractionKind {
    Trade(TradeNegotiation),
    Ask(KnowledgeSharingInteraction),
}

#[derive(Component, Debug, Clone, Copy)]
pub struct KnowledgeSharing {
    pub seller_of: ItemEnum,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct KnowledgeSharingInteraction {
    pub seller_of: ItemEnum,
    pub partner: Entity,
}

type InteractionId = usize;

#[derive(Component, Debug)]
pub struct ObtainKnowledgeTask {
    pub content: KnowledgeSharing,
    pub tried: Vec<Entity>,
    pub current_interaction: Option<(InteractionId, Entity)>,
}


impl ObtainKnowledgeTask {
    pub fn new(knowledge: KnowledgeSharing) -> Self {
        Self {
            content: knowledge,
            tried: vec![],
            current_interaction: None,
        }
    }
}
