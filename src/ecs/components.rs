use bevy::{
    ecs::{component::Component, entity::Entity},
    math::Vec3,
};
use rand::random;

pub trait ActionMarker {
    fn should_set_idle_at_completion(&self) -> bool;
    fn set_idle_at_completion(&mut self, value: bool);
}

pub trait DurationAction {
    fn get_resting_duration(&self) -> f32;
    fn progress(&mut self, time: f32);
}

pub trait TimeoutAction {
    fn set_timed_out(&mut self);
    fn is_timed_out(&self) -> bool;
}

#[derive(Component, Default)]
pub struct Idle;

pub type InteractionId = u32;

#[derive(Component, Debug)]
pub struct Interacting {
    pub id: InteractionId,
    pub source: Entity,
    pub target: Entity,
    resting_duration: f32,
    timed_out: bool,
    status: InteractingStatusEnum,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractingStatusEnum {
    #[default]
    Waiting,
    Ready,
}

impl Interacting {
    pub fn new_with_id(id: InteractionId, source: Entity, target: Entity) -> Self {
        Self {
            id,
            target,
            source,
            timed_out: false,
            resting_duration: 10.,
            status: InteractingStatusEnum::Waiting
        }
    }

    pub fn is_waiting(&self) -> bool {
        self.status == InteractingStatusEnum::Waiting
    }

    pub fn is_ready(&self) -> bool {
        self.status == InteractingStatusEnum::Ready
    }

    pub fn set_ready(&mut self) {
        self.status = InteractingStatusEnum::Ready
    }
}

impl TimeoutAction for Interacting {
    fn set_timed_out(&mut self) {
        self.timed_out = true;
    }

    fn is_timed_out(&self) -> bool {
        self.timed_out
    }
}

impl DurationAction for Interacting {
    fn get_resting_duration(&self) -> f32 {
        self.resting_duration
    }
    fn progress(&mut self, time: f32) {
        self.resting_duration -= time;
    }
}

#[derive(Component, Debug)]
pub struct WaitingInteraction {
    resting_duration: f32,
    timed_out: bool,
    pub id: InteractionId,
    pub source: Entity,
    pub target: Entity,
}

impl WaitingInteraction {
    pub fn new(source: Entity, target: Entity) -> Self {
        Self {
            id: random(),
            resting_duration: 5.,
            source,
            target,
            timed_out: false
        }
    }

    pub fn new_with_duration(source: Entity, target: Entity, resting_duration: f32) -> Self {
        Self {
            id: random(),
            resting_duration,
            target,
            source,
            timed_out: false
        }
    }
}

impl TimeoutAction for WaitingInteraction {
    fn set_timed_out(&mut self) {
        self.timed_out = true;
    }

    fn is_timed_out(&self) -> bool {
        self.timed_out
    }
}

impl DurationAction for WaitingInteraction {
    fn get_resting_duration(&self) -> f32 {
        self.resting_duration
    }
    fn progress(&mut self, time: f32) {
        self.resting_duration -= time;
    }
}

#[derive(Component, Debug)]
pub struct Walking {
    pub destination: Vec3,
    should_set_idle: bool,
}

impl Walking {
    pub fn new(destination: Vec3) -> Self {
        Self {
            destination,
            should_set_idle: true,
        }
    }

    pub fn new_without_idle(destination: Vec3) -> Self {
        Self {
            destination,
            should_set_idle: false,
        }
    }
}

impl ActionMarker for Walking {
    fn set_idle_at_completion(&mut self, value: bool) {
        self.should_set_idle = value;
    }
    fn should_set_idle_at_completion(&self) -> bool {
        self.should_set_idle
    }
}
