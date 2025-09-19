use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query},
    },
    math::Vec3,
    transform::components::Transform,
};

use crate::ecs::{
    components::{Idle, Walking},
    trade::components::Selling,
};

#[derive(Component, Default)]
pub struct SellerRole {
    pub location: Vec3,
}

pub fn handle_idle_sellers(
    query: Query<(Entity, &Transform, &SellerRole), (With<SellerRole>, With<Idle>)>,
    mut commands: Commands,
) {
    for (entity, &transform, seller_role) in &query {
        if seller_role.location.distance(transform.translation) > 50. {
            commands
                .entity(entity)
                .insert(Walking::new(seller_role.location))
                .remove::<Idle>();
        } else {
            commands.entity(entity).insert(Selling::new()).remove::<Idle>();
        }
    }
}
