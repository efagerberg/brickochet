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
    pub driver: assets::EffectDriver,
    pub start: f32,
    pub end: f32,
    pub last_keyframe_index: Option<usize>,
    pub keyframes: Vec<assets::Keyframe<f32>>,
}

#[derive(Component)]
pub struct RicochetCurveEffectState {
    pub driver: assets::EffectDriver,
    pub start: f32,
    pub end: f32,
    pub last_keyframe_index: Option<usize>,
    pub keyframes: Vec<assets::Keyframe<Vec2>>,
}

#[derive(Component)]
pub struct RicochetSizeEffectState {
    pub driver: assets::EffectDriver,
    pub start: f32,
    pub end: f32,
    pub last_keyframe_index: Option<usize>,
    pub keyframes: Vec<assets::Keyframe<f32>>,
}
