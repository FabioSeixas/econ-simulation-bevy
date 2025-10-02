use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct ChangeSelectedEntity {
    pub target: Entity,
}
