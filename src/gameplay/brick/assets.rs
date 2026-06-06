use bevy::prelude::*;
use serde;

use crate::gameplay::brick::key_frames;

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath, Clone)]
pub struct BrickAsset {
    pub name: String,
    pub health: u8,
    pub icon: String,
    pub collision_sfx: Option<String>,
    pub ricochet_effect: Option<RicochetEffectDef>,
}

#[derive(Clone, serde::Deserialize)]
pub enum EffectDriver {
    Time { duration_seconds: f32 },
    DistanceToPlayer,
}

#[derive(Clone, serde::Deserialize)]
pub struct RicochetEffectDef {
    pub driver: EffectDriver,
    pub attribute: RicochetEffectAttribute,
}

#[derive(Clone, serde::Deserialize)]
pub enum RicochetEffectAttribute {
    Speed(ScalarCurve),
    Curve(Vec2Curve),
    Size(ScalarCurve),
}

#[derive(Clone, serde::Deserialize)]
pub struct ScalarCurve {
    pub key_frames: Vec<key_frames::KeyFrame<f32>>,
}

#[derive(Clone, serde::Deserialize)]
pub struct Vec2Curve {
    pub key_frames: Vec<key_frames::KeyFrame<Vec2>>,
}
