use bevy::{ecs::component::Component, math::Vec3};

use crate::{core::item::ItemEnum, ecs::utils::get_random_vec3};

pub trait ActionMarker {
    fn should_set_idle_at_completion(&self) -> bool;
    fn set_idle_at_completion(&mut self, value: bool);
}

pub trait DurationActionMarker {
    fn get_resting_duration(&self) -> f32;
    fn progress(&mut self, time: f32);
}

#[derive(Component, Default)]
pub struct Idle;

#[derive(Component, Default, Debug)]
pub struct Interacting;

#[derive(Component, Default, Debug)]
pub struct WaitingInteraction {
    resting_duration: f32,
}

impl WaitingInteraction {
    pub fn new() -> Self {
        Self {
            resting_duration: 5.,
        }
    }
}

impl DurationActionMarker for WaitingInteraction {
    fn get_resting_duration(&self) -> f32 {
        self.resting_duration
    }
    fn progress(&mut self, time: f32) {
        self.resting_duration -= time;
    }
}

#[derive(Component, Debug)]
pub struct InteractionWalking {
    pub destination: Vec3,
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
}

impl ActionMarker for Walking {
    fn set_idle_at_completion(&mut self, value: bool) {
        self.should_set_idle = value;
    }
    fn should_set_idle_at_completion(&self) -> bool {
        self.should_set_idle
    }
}

#[derive(Component)]
pub struct Consuming {
    resting_duration: f32,
    pub item: ItemEnum,
    pub qty: usize,
}

impl Consuming {
    pub fn new(item: ItemEnum, qty: usize) -> Self {
        Self {
            item,
            qty,
            resting_duration: 5. * (qty as f32),
        }
    }
}

impl DurationActionMarker for Consuming {
    fn get_resting_duration(&self) -> f32 {
        self.resting_duration
    }
    fn progress(&mut self, time: f32) {
        self.resting_duration -= time;
    }
}

#[derive(Component)]
pub struct ConsumeTask {
    pub location: Vec3,
    pub item: ItemEnum,
    pub qty: usize,
}

impl ConsumeTask {
    pub fn new(item: ItemEnum, qty: usize) -> Self {
        Self {
            location: get_random_vec3(),
            item,
            qty,
        }
    }
}
