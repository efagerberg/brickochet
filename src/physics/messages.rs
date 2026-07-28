use crate::physics::math;
use bevy::prelude::*;

#[derive(Message, Debug, PartialEq, Copy, Clone)]
pub struct CollisionMessage {
    pub a: Entity,
    pub b: Entity,
    pub hit: math::SweepHit,
}
