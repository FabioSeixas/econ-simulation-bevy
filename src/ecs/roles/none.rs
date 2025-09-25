use bevy::ecs::{
    component::Component,
    entity::Entity,
    event::EventWriter,
    query::With,
    system::{Commands, Query},
};

use crate::ecs::{
    components::{Idle, Walking},
    logs::AddLogEntry,
    utils::get_random_vec3,
};

#[derive(Component, Default)]
pub struct NoneRole;

pub fn handle_idle_none_role(
    query: Query<Entity, (With<NoneRole>, With<Idle>)>,
    mut commands: Commands,
    mut add_log_writer: EventWriter<AddLogEntry>,
) {
    for entity in &query {
        add_log_writer.send(AddLogEntry::new(entity, "Role -> Start Walking"));
        commands
            .entity(entity)
            .insert(Walking::new(get_random_vec3()))
            .remove::<Idle>();
    }
}
