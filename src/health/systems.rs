use bevy::prelude::*;

use crate::health;

pub fn handle_health_changed(
    mut health_changed_messages: MessageReader<health::messages::HealChangedMessage>,
    mut death_messages: MessageWriter<health::messages::DeathMessage>,
    mut health_query: Query<&mut health::components::Health>,
) {
    for message in health_changed_messages.read() {
        if let Ok(mut health) = health_query.get_mut(message.entity) {
            let new_health =
                (health.current as i16 + message.delta).clamp(0, health.max as i16) as u8;
            health.current = new_health;
            if health.current == 0 {
                death_messages.write(health::messages::DeathMessage {
                    entity: message.entity,
                });
            }
        }
    }
}

pub fn handle_death(
    mut messages: MessageReader<health::messages::DeathMessage>,
    mut commands: Commands,
) {
    for message in messages.read() {
        commands.entity(message.entity).despawn();
    }
}
