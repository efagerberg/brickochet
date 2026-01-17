use bevy::prelude::*;

#[derive(Message)]
pub struct CollisionMessage {
    pub a: Entity,
    pub b: Entity,
}
