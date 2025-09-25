use bevy::prelude::*;

use crate::ecs::roles::none::*;
use crate::ecs::roles::seller::*;

pub struct RolesPlugin;

impl Plugin for RolesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_idle_sellers, handle_idle_none_role))
            .add_observer(handle_selling_removed_from_seller);
    }
}
