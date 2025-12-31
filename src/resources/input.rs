use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct MousePosition(pub Option<Vec2>);
