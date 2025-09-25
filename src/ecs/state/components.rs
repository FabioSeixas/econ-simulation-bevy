use bevy::prelude::*;

// ====================
// === Task layer ===
// ====================
#[derive(Component, Default)]
pub enum TaskState {
    #[default]
    None,
    Buy,
    Consume,
}

// ====================
// === Action layer ===
// ====================
#[derive(Component, Default)]
pub enum ActionState {
    #[default]
    None,
    Walk,
    Buy,
    Sell,
    Talk,
}

// ====================
// === Interaction layer ===
// ====================
#[derive(Component, Default)]
pub enum InteractionState {
    #[default]
    None,
    Trade,
    Talk,
}
