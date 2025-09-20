use bevy::ecs::{component::Component, entity::Entity, event::Event};

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub description: String,
}

#[derive(Component)]
pub struct AgentLogs {
    logs: Vec<LogEntry>,
}

impl AgentLogs {
    pub fn new() -> Self {
        Self { logs: vec![] }
    }

    pub fn add(&mut self, entry: LogEntry) {
        self.logs.push(entry)
    }

    pub fn list(&self) -> &Vec<LogEntry> {
        &self.logs
    }
}

#[derive(Event, Debug)]
pub struct AddLogEntry {
    pub target: Entity,
    pub entry: LogEntry,
}

impl AddLogEntry {
    pub fn new(target: Entity, description: &str) -> Self {
        Self {
            target,
            entry: LogEntry {
                description: description.to_string(),
            },
        }
    }
}
