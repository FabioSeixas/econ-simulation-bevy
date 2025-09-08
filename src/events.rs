use bevy::prelude::{Event, Entity};

use crate::task::Task;

#[derive(Debug)]
pub enum TaskEventStatus {
    COMPLETED
}

#[derive(Event, Debug)]
pub struct TaskEvent {
    pub target: Entity,
    pub task: Task,
    pub status: TaskEventStatus,
}
