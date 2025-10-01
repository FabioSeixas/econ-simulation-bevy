use bevy::prelude::*;

use crate::{
    ecs::talk::{
        events::{TalkFinishedWithFailure, TalkFinishedWithSuccess},
        interaction::plugin::TalkInteractionPlugin,
        task::systems::*,
    },
    GameState,
};

pub struct TalkPlugin;

impl Plugin for TalkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TalkInteractionPlugin)
            .add_event::<TalkFinishedWithSuccess>()
            .add_event::<TalkFinishedWithFailure>()
            .add_systems(
                Update,
                (
                    handle_added_talk_task,
                    handle_waiting_while_talk_task,
                    handle_get_close_to_target_while_talk_task,
                )
                    .run_if(in_state(GameState::Running)),
            )
            .add_observer(handle_talk_failure)
            .add_observer(handle_talk_success);
    }
}
