use bevy::{color::Color, ecs::{entity::Entity, system::Resource}};

#[derive(Resource, Default)]
pub struct SelectedAgent {
    pub entity: Option<(Entity, Color)>,
}
