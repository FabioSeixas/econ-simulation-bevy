use bevy::prelude::*;

use crate::core::item::ItemEnum;

#[derive(Component, Debug, Clone, Copy)]
pub struct KnowledgeSharingInteraction {
    pub seller_of: ItemEnum,
    pub partner: Entity,
}
