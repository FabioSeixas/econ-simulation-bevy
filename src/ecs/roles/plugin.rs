use bevy::app::{App, Plugin, Update};

use crate::ecs::roles::none::handle_idle_none_role;
use crate::ecs::roles::seller::handle_idle_sellers;

pub struct RolesPlugin;

impl Plugin for RolesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_idle_none_role, handle_idle_sellers));
    }
}
