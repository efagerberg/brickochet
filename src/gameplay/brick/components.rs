use bevy::prelude::*;

use crate::gameplay::brick::assets;

#[derive(Component)]
pub struct Brick;

#[derive(Component)]
pub struct RicochetEffectConfig {
    pub definition: assets::RicochetEffectDef,
}

#[derive(Component)]
pub struct RicochetSpeedEffectState {
    pub start: f32,
    pub end: f32,
    pub last_keyframe_index: Option<usize>,
    pub keyframes: Vec<assets::Keyframe<f32>>,
}

#[derive(Component)]
pub struct RicochetCurveEffectState {
    pub start: f32,
    pub end: f32,
    pub last_keyframe_index: Option<usize>,
    pub keyframes: Vec<assets::Keyframe<Vec2>>,
}
