use bevy::prelude::*;

use crate::gameplay::brick::assets;

#[derive(Component)]
pub struct Brick;

#[derive(Component)]
pub struct RicochetEffect {
    pub driver: assets::EffectDriver,
    pub definition: assets::RicochetEffectDef,
}

#[derive(Component)]
pub struct RicochetEffectPresentation {
    pub sfx: Option<Handle<AudioSource>>,
}
