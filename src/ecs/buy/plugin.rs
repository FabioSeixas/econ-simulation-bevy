use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
    state::condition::in_state,
};

use crate::{
    ecs::buy::{
        actions::systems::{handle_buy_action, handle_waiting_interaction_timed_out},
        tasks::systems::{handle_buy_task, handle_buying_failed},
    },
    GameState,
};

pub struct BuyPlugin;

impl Plugin for BuyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_buy_task, handle_buy_action).run_if(in_state(GameState::Running)),
        )
        .add_observer(handle_buying_failed)
        .add_observer(handle_waiting_interaction_timed_out);
    }
}
