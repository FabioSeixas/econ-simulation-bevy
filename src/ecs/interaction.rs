use std::collections::VecDeque;

use bevy::ecs::{component::Component, entity::Entity, event::Event};

use crate::ecs::{
    components::InteractionId, talk::interaction::components::KnowledgeSharingInteraction,
    trade::components::TradeNegotiation,
};

#[derive(Event, Debug)]
pub struct InteractionTimedOut {
    pub id: InteractionId,
}

#[derive(Event, Debug)]
pub struct WaitingInteractionTimedOut {
    pub id: InteractionId,
    pub source: Entity,
    pub target: Entity,
}

#[derive(Component)]
pub struct AgentInteractionQueue {
    queue: VecDeque<AgentInteractionItem>,
}

impl AgentInteractionQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn add(&mut self, item: AgentInteractionItem) {
        self.queue.push_back(item);
    }

    pub fn rm_id(&mut self, rm_id: InteractionId) {
        self.queue.retain(|event| event.id != rm_id);
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn get_first(&mut self) -> Option<&AgentInteractionItem> {
        match self.queue.front() {
            None => None,
            Some(v) => Some(v),
        }
    }

    pub fn pop_first(&mut self) -> Option<AgentInteractionItem> {
        match self.queue.pop_front() {
            None => None,
            Some(v) => Some(v),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AgentInteractionItem {
    pub id: InteractionId,
    pub kind: AgentInteractionKind,
}

#[derive(Debug, Clone)]
pub enum AgentInteractionKind {
    Trade(TradeNegotiation),
    Ask(KnowledgeSharingInteraction),
}
