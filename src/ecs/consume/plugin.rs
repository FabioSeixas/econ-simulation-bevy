use bevy::prelude::*;

use crate::ecs::consume::{actions::systems::handle_consuming_action, tasks::systems::*};

pub struct ConsumePlugin;

impl Plugin for ConsumePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_consuming_action, handle_consume_task))
            .add_systems(PostUpdate, handle_pause_while_consume_task)
            .add_observer(handle_resume_consume_task_on_interacting_removed)
            .add_observer(handle_resume_consume_task_on_consuming_removed)
            .add_observer(handle_resume_consume_task_on_walking_removed);
    }
}
