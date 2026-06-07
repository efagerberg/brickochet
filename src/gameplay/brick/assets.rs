use bevy::prelude::*;
use serde;

use crate::gameplay::brick::key_frames;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum LightFXType {
    Pulse {
        speed: f32,
        color: [f32; 3],
    },
    Wave {
        speed: f32,
        color: [f32; 3],
    },
    Strobe {
        speed: f32,
        color: [f32; 3],
    },
    Interference {
        freq1: f32,
        freq2: f32,
        color: [f32; 3],
    },
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct LightFX {
    pub effect_type: LightFXType,
    pub intensity: f32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct CollisionSFX {
    pub path: String,
    pub volume: f32,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath, Clone)]
pub struct BrickAsset {
    pub name: String,
    pub health: u8,
    pub collision_sfx: Option<CollisionSFX>,
    pub light_fx: Option<LightFX>,
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
