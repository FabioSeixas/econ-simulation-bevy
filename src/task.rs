use bevy::{math::Vec3, prelude::Entity};

use crate::ActionType;

#[derive(Debug, Clone)]
pub struct Task {
    pub _id: u8,
    pub _name: String,
    pub _where: Vec3,
    pub _duration: u32,
    pub _action_type: ActionType,
    pub _target: Option<Entity>
    // pub category: Vec3,
    // pub callback:
}

impl Task {
    pub fn new() -> Self {
        Self {
            _duration: 0,
            _where: Vec3::new(0., 0., 0.),
            _name: String::from(""),
            _action_type: ActionType::SLEEP,
            _id: 1,
            _target: None
        }
    }
}
