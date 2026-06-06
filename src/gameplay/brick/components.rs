use bevy::prelude::*;

use crate::gameplay::brick::{assets, key_frames};

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
    pub key_frames: Vec<key_frames::KeyFrame<f32>>,
}

#[derive(Component)]
pub struct RicochetCurveEffectState {
    pub driver: assets::EffectDriver,
    pub start: f32,
    pub end: f32,
    pub key_frames: Vec<key_frames::KeyFrame<Vec2>>,
}

#[derive(Component)]
pub struct RicochetSizeEffectState {
    pub driver: assets::EffectDriver,
    pub start: f32,
    pub end: f32,
    pub key_frames: Vec<key_frames::KeyFrame<f32>>,
}
