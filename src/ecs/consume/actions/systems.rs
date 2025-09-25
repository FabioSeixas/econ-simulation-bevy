use bevy::prelude::*;

use crate::ecs::agent::Agent;
use crate::ecs::components::*;
use crate::ecs::consume::actions::components::Consuming;
use crate::ecs::logs::*;

pub fn handle_consuming_action(
    mut query: Query<(Entity, &mut Agent, &mut Consuming), Without<Interacting>>,
    time: Res<Time>,
    mut commands: Commands,
    mut add_log_writer: EventWriter<AddLogEntry>,
) {
    for (entity, mut agent, mut consuming) in &mut query {
        if consuming.get_resting_duration() > 0. {
            consuming.progress(time.delta_secs());
            continue;
        }

        let item = consuming.item.clone();
        if item.is_food() {
            add_log_writer.send(AddLogEntry::new(entity, "Consume (eat) done"));
            agent.satisfy_hungry();
        }

        if item.is_liquid() {
            add_log_writer.send(AddLogEntry::new(entity, "Consume (drink) done"));
            agent.satisfy_thirsty();
        }
        agent.inventory.remove(item, consuming.qty);

        commands.entity(entity).remove::<Consuming>();
    }
}
