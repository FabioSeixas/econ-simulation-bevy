use bevy::app::{App, Plugin, Update};

use crate::ecs::talk::{
    interaction::plugin::TalkInteractionPlugin,
    systems::{handle_talk_task_system, start_talk_interaction_system},
};

pub struct TalkPlugin;

impl Plugin for TalkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TalkInteractionPlugin)
            .add_systems(Update, (handle_talk_task_system, start_talk_interaction_system));
    }
}
