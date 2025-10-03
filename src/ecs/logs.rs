use std::time::{Duration, SystemTime, UNIX_EPOCH};

use bevy::ecs::{component::Component, entity::Entity, event::Event};

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub description: String,
    pub time: Duration,
    pub frame: u32
}

#[derive(Component)]
pub struct AgentLogs {
    logs: Vec<LogEntry>,
}

impl AgentLogs {
    pub fn new() -> Self {
        Self { logs: vec![] }
    }

    pub fn add(&mut self, description: &String, frame: u32) {
        if description.contains("Start Consuming") {
            if let Some(v) = self.logs.last() {
                if v.description.contains("Start Consuming") {
                    for log in self.logs.clone() {
                        println!("{:?} at {:?}", log.description, log.time);
                    }
                    panic!("several start consuming");
                }
            }
        }
        self.logs.push(LogEntry {
            description: description.clone(),
            time: SystemTime::now().duration_since(UNIX_EPOCH).ok().unwrap(),
            frame
        });

        if self.logs.len() > 100 && self.logs.len() % 100 == 0 {
            self.logs.drain(0..100);
        }
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
