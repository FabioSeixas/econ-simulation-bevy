use bevy::prelude::*;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{core::item::ItemEnum, ecs::agent::Agent};

#[derive(Component)]
pub struct AgentKnowledge {
    pub agent: Entity,
    pub knowledge: Arc<RwLock<BaseKnowledge>>,
}

impl AgentKnowledge {
    pub fn get_sellers_of(&self, item: &ItemEnum) -> Vec<Entity> {
        match self.knowledge.read().unwrap().sellers.get(&item) {
            None => vec![],
            Some(v) => v.clone(),
        }
    }
}

/// The singleton knowledge base content
#[derive(Debug)]
pub struct BaseKnowledge {
    pub sellers: HashMap<ItemEnum, Vec<Entity>>,
}

/// Resource that holds the shared Arc<RwLock<_>>
#[derive(Resource, Clone)]
pub struct SharedKnowledge(Arc<RwLock<BaseKnowledge>>);

impl SharedKnowledge {
    pub fn clone_base_knowledge(&self) -> Arc<RwLock<BaseKnowledge>> {
        self.0.clone()
    }

    pub fn add_seller(&mut self, item: ItemEnum, seller: Entity) {
        if let Some(sellers) = self
            .0
            .write()
            .expect("try to write on sellers knowledge fail")
            .sellers
            .get_mut(&item)
        {
            sellers.push(seller);
        }
    }
}

/// The plugin that sets everything up
pub struct KnowledgePlugin;

fn start_sellers_map() -> HashMap<ItemEnum, Vec<Entity>> {
    let mut map = HashMap::new();

    for item in ItemEnum::ALL {
        map.insert(item, vec![]);
    }

    map
}

impl Plugin for KnowledgePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SharedKnowledge(Arc::new(RwLock::new(BaseKnowledge {
            sellers: start_sellers_map(),
        }))));

        // Add systems to inject knowledge and allow agents to use it
        app.add_systems(Update, attach_agent_knowledge);
    }
}

fn attach_agent_knowledge(
    mut commands: Commands,
    shared: Res<SharedKnowledge>,
    query: Query<Entity, (With<Agent>, Without<AgentKnowledge>)>, // example: any named entity
) {
    for entity in &query {
        commands.entity(entity).insert(AgentKnowledge {
            knowledge: shared.clone_base_knowledge(),
            agent: entity,
        });
    }
}
