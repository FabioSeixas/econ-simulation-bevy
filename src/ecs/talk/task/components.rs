use bevy::prelude::*;

use crate::ecs::{components::Task, talk::action::components::TalkAction};

type InteractionId = usize;

#[derive(Component, Debug)]
#[require(Task)]
pub struct TalkTask {
    pub content: TalkAction,
    pub tried: Vec<Entity>,
    pub current_interaction: Option<(InteractionId, Entity)>,
}

impl TalkTask {
    pub fn new(content: TalkAction) -> Self {
        Self {
            content,
            tried: vec![],
            current_interaction: None,
        }
    }
}
