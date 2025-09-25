use bevy::prelude::*;

use crate::ecs::components::*;
use crate::ecs::consume::actions::components::Consuming;
use crate::ecs::consume::tasks::components::ConsumeTask;
use crate::ecs::logs::*;
use crate::ecs::traits::*;

pub fn handle_consume_task(
    mut query: Query<(Entity, &Transform, &ConsumeTask)>,
    mut commands: Commands,
    mut add_log_writer: EventWriter<AddLogEntry>,
) {
    for (entity, transform, consume_task) in &mut query {
        if consume_task.is_paused() {
            continue;
        } else if consume_task.location.distance(transform.translation) > 50. {
            add_log_writer.send(AddLogEntry::new(entity, "Start Walking to consume"));
            let walking = Walking::new_with_not_idle(consume_task.location);
            commands.entity(entity).insert(walking);
        } else {
            add_log_writer.send(AddLogEntry::new(entity, "Start Consuming"));
            commands
                .entity(entity)
                .insert(Consuming::new(consume_task.item, consume_task.qty));
        }
    }
}

pub fn handle_pause_while_consume_task(
    mut query: Query<
        (
            &mut ConsumeTask,
            Option<&Interacting>,
            Option<&Consuming>,
            Option<&Walking>,
        ),
        Or<(Added<Interacting>, Added<Consuming>, Added<Walking>)>,
    >,
) {
    for (mut task, maybe_interacting, maybe_consuming, maybe_walking) in &mut query {
        if let Some(_) = maybe_interacting {
            task.pause(PauseReason::Interacting);
        } else if let Some(_) = maybe_consuming {
            task.pause(PauseReason::Consuming);
        } else if let Some(_) = maybe_walking {
            task.pause(PauseReason::Walking);
        }
    }
}

pub fn handle_resume_consume_task_on_interacting_removed(
    trigger: Trigger<OnRemove, Interacting>,
    mut query: Query<&mut ConsumeTask>,
) {
    if let Ok(mut task) = query.get_mut(trigger.entity()) {
        task.resume(PauseReason::Interacting);
    }
}

pub fn handle_resume_consume_task_on_walking_removed(
    trigger: Trigger<OnRemove, Walking>,
    mut query: Query<&mut ConsumeTask>,
) {
    if let Ok(mut task) = query.get_mut(trigger.entity()) {
        task.resume(PauseReason::Walking);
    }
}

pub fn handle_resume_consume_task_on_consuming_removed(
    trigger: Trigger<OnRemove, Consuming>,
    query: Query<&ConsumeTask>,
    mut commands: Commands,
) {
    if let Ok(_) = query.get(trigger.entity()) {
        commands
            .entity(trigger.entity())
            .insert(Idle)
            .remove::<ConsumeTask>();
    }
}
