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
            let walking = Walking::new_without_idle(consume_task.location);
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
            Entity,
            &mut ConsumeTask,
            Option<&Interacting>,
            Option<&Consuming>,
            Option<&Walking>,
        ),
        Or<(Added<Interacting>, Added<Consuming>, Added<Walking>)>,
    >,
    mut add_log_writer: EventWriter<AddLogEntry>,
) {
    for (entity, mut task, maybe_interacting, maybe_consuming, maybe_walking) in &mut query {
        if let Some(_) = maybe_interacting {
            add_log_writer.send(AddLogEntry::new(
                entity,
                "Pausing ConsumeTask due to Interacting",
            ));
            task.pause(PauseReason::Interacting);
        } 

        if let Some(_) = maybe_consuming {
            add_log_writer.send(AddLogEntry::new(
                entity,
                "Pausing ConsumeTask due to Consuming",
            ));
            task.pause(PauseReason::Consuming);
        } 

        if let Some(_) = maybe_walking {
            add_log_writer.send(AddLogEntry::new(
                entity,
                "Pausing ConsumeTask due to Walking",
            ));
            task.pause(PauseReason::Walking);
        }
    }
}

pub fn handle_resume_consume_task_on_interacting_removed(
    trigger: Trigger<OnRemove, Interacting>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut query: Query<&mut ConsumeTask>,
) {
    if let Ok(mut task) = query.get_mut(trigger.entity()) {
        add_log_writer.send(AddLogEntry::new(
            trigger.entity(),
            "Resuming ConsumeTask after Interacting",
        ));
        task.resume(PauseReason::Interacting);
    }
}

pub fn handle_resume_consume_task_on_walking_removed(
    trigger: Trigger<OnRemove, Walking>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut query: Query<&mut ConsumeTask>,
) {
    if let Ok(mut task) = query.get_mut(trigger.entity()) {
        add_log_writer.send(AddLogEntry::new(
            trigger.entity(),
            "Resuming ConsumeTask after Walking",
        ));
        task.resume(PauseReason::Walking);
    }
}

pub fn handle_resume_consume_task_on_consuming_removed(
    trigger: Trigger<OnRemove, Consuming>,
    query: Query<&ConsumeTask>,
    mut add_log_writer: EventWriter<AddLogEntry>,
    mut commands: Commands,
) {
    if let Ok(_) = query.get(trigger.entity()) {
        add_log_writer.send(AddLogEntry::new(
            trigger.entity(),
            "Ending ConsumeTask after Consuming",
        ));
        commands
            .entity(trigger.entity())
            .insert(Idle)
            .remove::<ConsumeTask>();
    }
}
