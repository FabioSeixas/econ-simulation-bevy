use std::time::{Duration, SystemTime, UNIX_EPOCH};

use bevy::ecs::{component::Component, entity::Entity, event::Event};

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub description: String,
    pub time: Duration,
}

#[derive(Component)]
pub struct AgentLogs {
    logs: Vec<LogEntry>,
}

impl AgentLogs {
    pub fn new() -> Self {
        Self { logs: vec![] }
    }

    pub fn add(&mut self, description: &String) {
        if description.contains("Start Consuming") {
            if let Some(v) = self.logs.last() {
                if v.description.contains("Start Consuming") {
                    println!("{:?}", self.logs);
                }
            }
        }
        self.logs.push(LogEntry {
            description: description.clone(),
            time: SystemTime::now().duration_since(UNIX_EPOCH).ok().unwrap(),
        })
    }

    pub fn list(&self) -> &Vec<LogEntry> {
        &self.logs
    }
}

#[derive(Event, Debug)]
pub struct AddLogEntry {
    pub target: Entity,
    pub description: String,
}

impl AddLogEntry {
    pub fn new(target: Entity, description: &str) -> Self {
        Self {
            target,
            description: description.to_string(),
        }
    }
}
