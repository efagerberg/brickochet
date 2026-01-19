use bevy::prelude::*;

#[derive(Message)]
pub struct CollisionMessage {
    pub a: Entity,
    pub b: Entity,
    pub normal: Vec3,
    pub contact_point: Vec3,
    pub penetration: f32,
}
