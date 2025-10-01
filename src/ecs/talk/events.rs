use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct TalkFinishedWithSuccess {
    pub target: Entity,
}

#[derive(Event, Debug)]
pub struct TalkFinishedWithFailure {
    pub target: Entity,
}
