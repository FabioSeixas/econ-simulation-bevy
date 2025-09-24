use std::collections::VecDeque;

use bevy::ecs::{component::Component, entity::Entity};

use crate::{core::item::ItemEnum, ecs::trade::components::TradeNegotiation};

#[derive(Component)]
pub struct AgentInteractionQueue {
    next_id: usize,
    queue: VecDeque<AgentInteractionEvent>,
}

impl AgentInteractionQueue {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            queue: VecDeque::new(),
        }
    }

    pub fn add(&mut self, kind: AgentInteractionKind) -> usize {
        self.queue.push_back(AgentInteractionEvent {
            id: self.next_id,
            kind,
        });
        self.next_id += 1;
        self.next_id
    }

    pub fn rm_id(&mut self, rm_id: usize) {
        self.queue.retain(|event| event.id != rm_id);
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn get_first(&mut self) -> Option<&AgentInteractionEvent> {
        match self.queue.front() {
            None => None,
            Some(v) => Some(v),
        }
    }

    pub fn pop_first(&mut self) -> Option<AgentInteractionEvent> {
        match self.queue.pop_front() {
            None => None,
            Some(v) => Some(v),
        }
    }
}

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
