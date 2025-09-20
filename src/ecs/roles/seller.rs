use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        query::{With, Without},
        system::{Commands, Query, Res},
    },
    math::Vec3,
    time::Time,
    transform::components::Transform,
};

use crate::ecs::{
    components::{DurationActionMarker, Idle, Interacting, Walking},
    trade::components::Selling,
};

#[derive(Component, Default)]
pub struct SellerRole {
    pub location: Vec3,
}

pub fn handle_idle_sellers(
    query: Query<
        (Entity, &Transform, &SellerRole),
        (With<SellerRole>, With<Idle>, Without<Selling>),
    >,
    mut commands: Commands,
) {
    for (entity, &transform, seller_role) in &query {
        if seller_role.location.distance(transform.translation) > 50. {
            commands
                .entity(entity)
                .insert(Walking::new(seller_role.location))
                .remove::<Idle>();
        } else {
            commands
                .entity(entity)
                .insert(Selling::new())
                .remove::<Idle>();
        }
    }
}

pub fn handle_selling_action(
    mut query: Query<(Entity, &mut Selling), (With<Selling>, Without<Interacting>, Without<Idle>)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut selling) in &mut query {
        if selling.get_resting_duration() > 0. {
            selling.progress(time.delta_secs());
            continue;
        }

        commands.entity(entity).insert(Idle).remove::<Selling>();
    }
}
