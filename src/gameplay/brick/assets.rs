use bevy::prelude::*;
use serde;

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
}

#[derive(Clone, serde::Deserialize)]
pub struct Keyframe<T> {
    pub t: f32,
    pub value: T,
}

#[derive(Clone, serde::Deserialize)]
pub struct ScalarCurve {
    pub keyframes: Vec<Keyframe<f32>>,
}

#[derive(Clone, serde::Deserialize)]
pub struct Vec2Curve {
    pub keyframes: Vec<Keyframe<Vec2>>,
}
