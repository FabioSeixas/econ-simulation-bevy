use bevy::prelude::*;

use crate::core::item::ItemEnum;

#[derive(Component, Debug, Clone, Copy)]
pub struct TalkAction {
    pub seller_of: ItemEnum,
}
