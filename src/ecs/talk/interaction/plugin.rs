use bevy::app::{App, Plugin, Update};

use crate::ecs::talk::interaction::events::{SendKnowledgeEvent, ShareKnowledgeFinalizedEvent};
use crate::ecs::talk::interaction::systems::{
    handle_knowlegde_share_finalized_system, handle_knowlegde_share_request_system,
    share_knowledge_finalized_system,
};

pub struct TalkInteractionPlugin;

impl Plugin for TalkInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SendKnowledgeEvent>()
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
