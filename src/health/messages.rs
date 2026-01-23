use bevy::prelude::*;

#[derive(Message)]
pub struct HealChangedMessage {
    pub entity: Entity,
    pub delta: i16,
}


#[derive(Message)]
pub struct DeathMessage {
    pub entity: Entity,
}
