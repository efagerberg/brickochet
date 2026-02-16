use bevy::prelude::*;
use serde;

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath, Clone)]
pub struct BrickAsset {
    pub name: String,
    pub health: u8,
    pub icon: String,
    pub ricochet: RicochetEffectAsset,
}

#[derive(Clone, serde::Deserialize)]
pub struct RicochetEffectAsset {
    pub driver: EffectDriver,
    pub presentation: RicochetEffectPresentation,
    pub effect: RicochetEffectDef,
}

#[derive(Clone, serde::Deserialize)]
pub enum EffectDriver {
    Time { duration_seconds: f32 },
    Distance { total_meters: f32 },
}

#[derive(Clone, serde::Deserialize)]
pub struct RicochetEffectPresentation {
    pub sfx: Option<String>,
}

#[derive(Clone, serde::Deserialize)]
pub enum RicochetEffectDef {
    Speed(ScalarCurve),
    Size(ScalarCurve),
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
