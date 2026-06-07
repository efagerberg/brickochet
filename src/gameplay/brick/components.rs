use bevy::prelude::*;

use crate::gameplay::brick::{assets, key_frames};

#[derive(Component)]
pub struct Brick;

#[derive(Component)]
pub struct RicochetEffectConfig {
    pub definition: assets::RicochetEffectDef,
}

#[derive(Component)]
pub struct RicochetEffect<T> {
    pub driver: assets::EffectDriver,
    pub start: f32,
    pub end: f32,
    pub key_frames: Vec<key_frames::KeyFrame<T>>,
}

#[derive(Component)]
pub struct RicochetSpeedEffect(pub RicochetEffect<f32>);

#[derive(Component)]
pub struct RicochetCurveEffect(pub RicochetEffect<Vec2>);

#[derive(Component)]
pub struct RicochetSizeEffect(pub RicochetEffect<f32>);
