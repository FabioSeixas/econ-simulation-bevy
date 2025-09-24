use bevy::app::{App, Plugin, Update};

use crate::ecs::consume::{
    actions::systems::handle_consuming_action, tasks::systems::handle_consume_task,
};

pub struct ConsumePlugin;

impl Plugin for ConsumePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_consuming_action, handle_consume_task));
    }
}
