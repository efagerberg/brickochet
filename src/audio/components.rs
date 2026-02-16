use bevy::prelude::*;

#[derive(Component)]
pub struct CollisionSFX(pub Handle<AudioSource>);
