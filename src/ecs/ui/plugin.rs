use bevy::app::{App, Plugin, Update};
use bevy_egui::EguiPlugin;

use crate::ecs::ui::{
    resources::SelectedAgent,
    systems::{agent_selection_system, agent_ui_panel_system},
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .init_resource::<SelectedAgent>()
            .add_systems(Update, (agent_selection_system, agent_ui_panel_system));
    }
}
