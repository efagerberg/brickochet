use std::collections::HashSet;

use bevy::{asset::LoadedFolder, prelude::*};

// Load folder resources to keep handle references. Used for preloading assets
#[derive(Resource)]
#[allow(dead_code)]
pub struct LoadedBrickFolder(pub Handle<LoadedFolder>);

#[derive(Resource)]
#[allow(dead_code)]
pub struct LoadedAudioFolder(pub Handle<LoadedFolder>);

#[derive(Resource)]
#[allow(dead_code)]
pub struct LoadedTextureFolder(pub Handle<LoadedFolder>);

#[derive(Resource)]
pub struct AssetFoldersLeftToLoad(pub HashSet<AssetId<LoadedFolder>>);
