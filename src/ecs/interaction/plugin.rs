use bevy::prelude::*;
use bevy::state::condition::in_state;

use crate::ecs::game_state::GameState;
use crate::ecs::interaction::common::systems::*;
use crate::ecs::interaction::source::systems::*;
use crate::ecs::interaction::target::systems::*;

pub struct BaseInteractionPlugin;

impl Plugin for BaseInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(receive_interaction_started_system)
            .add_observer(start_interaction_as_source_system)
            .add_observer(wait_finish_interaction_to_start_new_interaction_as_source_system)
            .add_observer(remove_timed_out_interaction_from_agent_queue)
            .add_observer(remove_timed_out_waiting_interaction_from_agent_queue)
            .add_observer(target_is_ready_to_start_interacting)
            .add_systems(
                First,
                (
                    interaction_timeout_system,
                    waiting_interaction_timeout_system,
                    handle_interaction_starting_for_source_system,
                    handle_interaction_starting_for_target_system,
                )
                    .chain()
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(
                PreUpdate,
                (check_agent_interaction_queue_system,).run_if(in_state(GameState::Running)),
            )
            .add_systems(
                Update,
                (
                    interaction_agents_move_on_system,
                )
                    .run_if(in_state(GameState::Running)),
            );
    }
}
