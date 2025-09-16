use bevy::ecs::component::Component;

#[derive(Component, Default)]
pub struct Idle;

#[derive(Component, Default, Debug)]
pub struct Interacting;
