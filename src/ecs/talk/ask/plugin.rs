use bevy::app::{App, Plugin, Update};

use crate::ecs::talk::ask::events::{ShareKnowledgeEvent, ShareKnowledgeFinalizedEvent};
use crate::ecs::talk::ask::systems::{
    handle_knowlegde_share_finalized_system, handle_knowlegde_share_request_system,
    share_knowledge_finalized_system,
};

pub struct AskPlugin;

impl Plugin for AskPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShareKnowledgeEvent>()
            .add_event::<ShareKnowledgeFinalizedEvent>()
            .add_systems(
                Update,
                (
                    share_knowledge_finalized_system,
                    handle_knowlegde_share_request_system,
                    handle_knowlegde_share_finalized_system,
                ),
            );
    }
}
