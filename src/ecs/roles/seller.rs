use bevy::prelude::*;

use crate::ecs::{
    components::{Idle, Walking},
    logs::AddLogEntry,
    sell::actions::components::Selling,
};

#[derive(Component, Default)]
pub struct SellerRole {
    pub location: Vec3,
}

pub fn handle_idle_sellers(
    query: Query<(Entity, &Transform, &SellerRole), (With<SellerRole>, With<Idle>)>,
    mut commands: Commands,
    mut add_log_writer: EventWriter<AddLogEntry>,
) {
    for (entity, &transform, seller_role) in &query {
        if seller_role.location.distance(transform.translation) > 50. {
            add_log_writer.send(AddLogEntry::new(
                entity,
                "Role -> Start Walking to sell location",
            ));
            commands
                .entity(entity)
                .insert(Walking::new(seller_role.location))
                .remove::<Idle>();
        } else {
            add_log_writer.send(AddLogEntry::new(entity, "Start Selling"));
            commands
                .entity(entity)
                .insert(Selling::new())
                .remove::<Idle>();
        }
    }
}

pub fn handle_selling_removed_from_seller(
    trigger: Trigger<OnRemove, Selling>,
    query: Query<&SellerRole>,
    mut commands: Commands,
) {
    if let Ok(_) = query.get(trigger.entity()) {
        commands.entity(trigger.entity()).insert(Idle);
    }
}
