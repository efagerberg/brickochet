use bevy::prelude::*;
use serde::Deserialize;

#[derive(Asset, TypePath, Deserialize)]
pub struct BrickAsset {
    pub health: i8,
    pub icon: String,
    pub ricochet: RicochetEffectAsset,
}

#[derive(Clone, Deserialize)]
pub struct RicochetEffectAsset {
    pub driver: EffectDriver,
    pub presentation: RicochetEffectPresentation,
    pub effect: RicochetEffectDef,
}

#[derive(Clone, Deserialize)]
pub enum EffectDriver {
    Time,
    Distance,
}

#[derive(Clone, Deserialize)]
pub struct RicochetEffectPresentation {
    pub sfx: Option<String>,
}

#[derive(Clone, Deserialize)]
pub enum RicochetEffectDef {
    Speed(ScalarCurve),
    Size(ScalarCurve),
    Curve(Vec2Curve),
}

#[derive(Clone, Deserialize)]
pub struct Keyframe<T> {
    pub t: f32,
    pub value: T,
}

#[derive(Clone, Deserialize)]
pub struct ScalarCurve {
    pub keyframes: Vec<Keyframe<f32>>,
}

#[derive(Clone, Deserialize)]
pub struct Vec2Curve {
    pub keyframes: Vec<Keyframe<Vec2>>,
}
