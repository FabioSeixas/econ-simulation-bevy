use bevy::{prelude::*, utils::HashSet};

use crate::core::item::ItemEnum;

type InteractionId = usize;

#[derive(Component, Debug)]
pub struct TalkTask {
    pub seller_of: ItemEnum,
    pub tried: HashSet<Entity>,
    pub current_interaction: Option<(InteractionId, Entity, Name)>,
}

impl TalkTask {
    pub fn new(seller_of: ItemEnum) -> Self {
        Self {
            seller_of,
            tried: HashSet::new(),
            current_interaction: None,
        }
    }
}
