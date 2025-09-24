use bevy::prelude::*;

use crate::ecs::components::*;
use crate::ecs::consume::actions::components::Consuming;
use crate::ecs::consume::tasks::components::ConsumeTask;
use crate::ecs::logs::*;

pub fn handle_consume_task(
    mut query: Query<
        (Entity, &Transform, &ConsumeTask),
        (
            With<ConsumeTask>,
            Without<Walking>,
            Without<Consuming>,
            Without<Idle>,
            Without<Interacting>,
        ),
    >,
    mut commands: Commands,
    mut add_log_writer: EventWriter<AddLogEntry>,
) {
    for (entity, transform, consume_task) in &mut query {
        if consume_task.location.distance(transform.translation) > 50. {
            add_log_writer.send(AddLogEntry::new(entity, "Start Walking to consume"));
            let mut walking = Walking::new(consume_task.location);
            walking.set_idle_at_completion(false);
            commands.entity(entity).insert(walking).remove::<Idle>();
        } else {
            add_log_writer.send(AddLogEntry::new(entity, "Start Consuming"));
            commands
                .entity(entity)
                .insert(Consuming::new(consume_task.item, consume_task.qty))
                .remove::<Idle>();
        }
    }
}
