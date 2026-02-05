use bevy::prelude::*;

use crate::{health, physics, rendering};

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

pub fn update_health_color(
    mut query: Query<(
        Entity,
        &health::components::Health,
        &health::components::HealthColors,
    )>,
    mut health_changed_messages: MessageReader<health::messages::HealChangedMessage>,
    mut material_colors_changed_messages: MessageWriter<
        rendering::messages::MaterialColorsChangedMessage,
    >,
) {
    for message in health_changed_messages.read() {
        if let Ok((entity, health, health_colors)) = query.get_mut(message.entity) {
            if health.current == 0 {
                continue;
            }
            let t = ((health.current as f32 - 1.0) / (health.max as f32 - 1.0)).clamp(0.0, 1.0);
            let new_color = Color::from(health_colors.min.mix(&health_colors.max, t));
            material_colors_changed_messages.write(
                rendering::messages::MaterialColorsChangedMessage {
                    entity,
                    base_color: Some(new_color),
                    emissive: None,
                },
            );
        }
    }
}

pub fn handle_collision(
    collided_query: Query<&health::components::ChangeOnCollision>,
    health_query: Query<&health::components::Health>,
    mut collision_messages: MessageReader<physics::messages::CollisionMessage>,
    mut health_changed_messages: MessageWriter<health::messages::HealChangedMessage>,
) {
    for message in collision_messages.read() {
        for &entity in [message.a, message.b].iter() {
            if let Ok(change_on_collision) = collided_query.get(entity) {
                for target in change_on_collision.affected_entities(entity) {
                    if !health_query.contains(target) {
                        continue;
                    }
                    health_changed_messages.write(health::messages::HealChangedMessage {
                        entity: target,
                        delta: change_on_collision.delta,
                    });
                }
            }
        }
    }
}
