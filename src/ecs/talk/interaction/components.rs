use bevy::prelude::*;

use crate::core::item::ItemEnum;

#[derive(Component, Debug, Clone)]
pub struct KnowledgeSharingInteraction {
    pub seller_of: ItemEnum,
    pub source: Entity,
    pub target: Entity,
    pub source_name: Name,
    pub target_name: Name,
    status: KnowledgeSharingInteractionEnum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnowledgeSharingInteractionEnum {
    WaitingSourceAgent,
    Started,
}

impl KnowledgeSharingInteraction {
    pub fn new(seller_of: ItemEnum, source: Entity, target: Entity, source_name: Name, target_name: Name) -> Self {
        Self {
            seller_of,
            source,
            target,
            source_name,
            target_name,
            status: KnowledgeSharingInteractionEnum::WaitingSourceAgent,
        }
    }

    pub fn is_waiting_target(&self) -> bool {
        self.status == KnowledgeSharingInteractionEnum::WaitingSourceAgent
    }

    pub fn start(&mut self) {
        self.status = KnowledgeSharingInteractionEnum::Started
    }
}
