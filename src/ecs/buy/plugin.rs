use bevy::app::{App, Plugin, Update};

use crate::ecs::buy::{actions::systems::handle_buy_action, tasks::systems::handle_buy_task};

pub struct BuyPlugin;

impl Plugin for BuyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_buy_task, handle_buy_action));
    }
}
