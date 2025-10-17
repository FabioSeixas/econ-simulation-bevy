use std::collections::VecDeque;

use bevy::ecs::component::Component;

use crate::ecs::{
    components::InteractionId, talk::interaction::components::KnowledgeSharingInteraction,
    trade::components::TradeNegotiation,
};

#[derive(Component)]
pub struct AgentInteractionQueue {
    received_as_target_queue: VecDeque<AgentInteractionItem>,
    start_as_source: Option<AgentInteractionItem>,
}

impl AgentInteractionQueue {
    pub fn new() -> Self {
        Self {
            received_as_target_queue: VecDeque::new(),
            start_as_source: None,
        }
    }

    // ====================
    // Methods to handle interactions ready to start as Source
    pub fn interaction_ready(&mut self, item: AgentInteractionItem) {
        self.start_as_source = Some(item);
    }

    pub fn get_ready_interaction(&self) -> Option<&AgentInteractionItem> {
        self.start_as_source.as_ref()
    }

    pub fn clean_ready_interaction(&mut self) {
        self.start_as_source = None;
    }

    // ====================
    // Methods to handle interactions as Target
    pub fn add(&mut self, item: AgentInteractionItem) {
        self.received_as_target_queue.push_back(item);
    }

    pub fn rm_id(&mut self, rm_id: InteractionId) {
        self.received_as_target_queue
            .retain(|event| event.id != rm_id);
    }

    pub fn list(&self) -> impl Iterator<Item = &AgentInteractionItem> {
        self.received_as_target_queue.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.received_as_target_queue.is_empty()
    }

    pub fn len(&self) -> usize {
        self.received_as_target_queue.len()
    }

    pub fn pop_first(&mut self) -> Option<AgentInteractionItem> {
        match self.received_as_target_queue.pop_front() {
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
