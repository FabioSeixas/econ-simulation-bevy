use bevy::ecs::{
    component::Component,
    entity::Entity,
    query::{With, Without},
    system::{Commands, Query},
};

use crate::ecs::{
    components::{Idle, Walking},
    utils::get_random_vec3,
};

#[derive(Component, Default)]
pub struct NoneRole;

pub fn handle_idle_none_role(
    query: Query<Entity, (With<NoneRole>, With<Idle>, Without<Walking>)>,
    mut commands: Commands,
) {
    for entity in &query {
        commands
            .entity(entity)
            .insert(Walking::new(get_random_vec3()))
            .remove::<Idle>();
    }
}
