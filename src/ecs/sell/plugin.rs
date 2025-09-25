use bevy::prelude::*;

use crate::ecs::sell::actions::systems::*;

pub struct SellPlugin;

impl Plugin for SellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_selling_action,
                handle_interaction_added_while_selling,
            ),
        )
        .add_observer(handle_interaction_removed_while_selling);
    }
}
