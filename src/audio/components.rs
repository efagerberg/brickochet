use bevy::prelude::*;

#[derive(Component)]
pub struct CollisionSFX {
    pub handle: Handle<AudioSource>,
    pub volume: f32,
}
