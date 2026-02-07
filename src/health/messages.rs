use bevy::prelude::*;

#[derive(Message, Copy, Clone, PartialEq, Debug)]
pub struct HealthChangedMessage {
    pub entity: Entity,
    pub delta: i16,
}

#[derive(Message, Copy, Clone, PartialEq, Debug)]
pub struct DeathMessage {
    pub entity: Entity,
}
