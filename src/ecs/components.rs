use bevy::ecs::component::Component;

#[derive(Component, Default)]
pub struct Idle;

#[derive(Component, Default, Debug)]
pub struct Interacting;

#[derive(Component, Default, Debug)]
pub struct Name {
    name: String,
}

impl Name {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}
