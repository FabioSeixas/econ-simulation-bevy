use bevy::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
};

use crate::{core::item::ItemEnum, ecs::agent::Agent};

#[derive(Component)]
pub struct AgentKnowledge {
    base_knowledge: Arc<RwLock<BaseKnowledge>>,
    known: HashSet<KnowledgeId>,
}

impl AgentKnowledge {
    pub fn get_sellers_of(&self, item: &ItemEnum) -> Vec<Entity> {
        let mut sellers = vec![];
        for id in self.known.iter() {
            if let Some(knowledge) = self
                .base_knowledge
                .read()
                .expect("fail to read on base knowledge")
                .get_fact(&id)
            {
                if let KnowledgeFact::SellerInfo { wares, entity, .. } = knowledge {
                    if wares.contains(item) {
                        sellers.push(entity)
                    }
                }
            }
        }
        sellers
    }
}

#[derive(Clone, Debug)]
pub enum KnowledgeFact {
    SellerInfo {
        entity: Entity,
        // name: String,
        location: Vec3,
        // We need to know what they sell to answer "where can I buy water?"
        wares: Vec<ItemEnum>,
    },
    // You could add other facts later, like:
    Recipe {
        output: ItemEnum,
        ingredients: Vec<ItemEnum>,
    },
    // PointOfInterest { name: String, location: Vec3 },
}

type KnowledgeId = u32;

/// The singleton knowledge base content
#[derive(Debug)]
pub struct BaseKnowledge {
    facts: HashMap<KnowledgeId, KnowledgeFact>,
    next_id: u32,
}

impl BaseKnowledge {
    pub fn new() -> Self {
        Self {
            facts: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add_fact(&mut self, fact: KnowledgeFact) -> KnowledgeId {
        // TODO: check if fact already exists
        self.facts.insert(self.next_id, fact);
        self.next_id += 1;
        self.next_id
    }

    // A method to look up the content of a fact by its ID.
    pub fn get_fact(&self, id: &KnowledgeId) -> Option<KnowledgeFact> {
        match self.facts.get(id) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }
}

#[derive(Resource, Clone)]
pub struct SharedKnowledge(Arc<RwLock<BaseKnowledge>>);

impl SharedKnowledge {
    pub fn clone_base_knowledge(&self) -> Arc<RwLock<BaseKnowledge>> {
        self.0.clone()
    }

    pub fn add_fact(&mut self, fact: KnowledgeFact) -> KnowledgeId {
        self.0
            .write()
            .expect("fail to write on base knowledge")
            .add_fact(fact)
    }

    pub fn get_all(&self) -> impl Iterator<Item = KnowledgeId> {
        let knowledge_lock = self.0.read().expect("fail to read on base_knowledge");
        let keys_vec: Vec<KnowledgeId> = knowledge_lock.facts.keys().cloned().collect();
        keys_vec.into_iter()
    }
}

/// The plugin that sets everything up
pub struct KnowledgePlugin;

impl Plugin for KnowledgePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SharedKnowledge(Arc::new(RwLock::new(BaseKnowledge::new()))));

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
            base_knowledge: shared.clone_base_knowledge(),
            known: HashSet::from_iter(shared.get_all()),
        });
    }
}
