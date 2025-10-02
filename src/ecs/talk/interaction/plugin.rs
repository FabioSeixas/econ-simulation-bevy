use bevy::app::{App, Plugin, Update};

use crate::ecs::talk::interaction::events::{
    SendKnowledgeEvent, ShareKnowledgeFinalizedEvent, StartTalkEvent,
};
use crate::ecs::talk::interaction::systems::{
    handle_interaction_timed_out, handle_knowlegde_share_requested_system,
    handle_knowlegde_share_started_system, handle_knowlegde_shared_system,
    share_knowledge_finalized_system,
};

pub struct TalkInteractionPlugin;

impl Plugin for TalkInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SendKnowledgeEvent>()
            .add_event::<ShareKnowledgeFinalizedEvent>()
            .add_event::<StartTalkEvent>()
            .add_observer(handle_interaction_timed_out)
            .add_systems(
                Update,
                (
                    handle_knowlegde_share_requested_system,
                    handle_knowlegde_share_started_system,
                    handle_knowlegde_shared_system,
                    share_knowledge_finalized_system,
                ),
            );
    }
}
