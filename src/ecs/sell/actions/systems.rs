use bevy::{
    ecs::{
        entity::Entity,
        observer::Trigger,
        query::Added,
        system::{Commands, Query, Res},
        world::OnRemove,
    },
    time::Time,
};

use crate::ecs::{
    components::{DurationAction, Interacting},
    sell::actions::components::Selling,
    traits::*,
};

pub fn handle_selling_action(
    mut query: Query<(Entity, &mut Selling)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut selling) in &mut query {
        if selling.is_paused() {
            continue;
        } else if selling.get_resting_duration() > 0. {
            selling.progress(time.delta_secs());
            continue;
        } else {
            commands.entity(entity).remove::<Selling>();
        }
    }
}

pub fn handle_interaction_added_while_selling(mut query: Query<&mut Selling, Added<Interacting>>) {
    for mut selling in &mut query {
        selling.pause(PauseReason::Interacting);
    }
}

pub fn handle_interaction_removed_while_selling(
    trigger: Trigger<OnRemove, Interacting>,
    mut query: Query<&mut Selling>,
) {
    let entity = trigger.entity();
    if let Ok(mut selling) = query.get_mut(entity) {
        selling.resume(PauseReason::Interacting);
    }
}
