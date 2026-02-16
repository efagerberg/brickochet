use std::collections::HashSet;

use bevy::{asset::LoadedFolder, prelude::*};

#[derive(Resource)]
pub struct LoadedBrickFolder(pub Handle<LoadedFolder>);

#[derive(Resource)]
pub struct LoadedAudioFolder(pub Handle<LoadedFolder>);

#[derive(Resource)]
pub struct LoadedTextureFolder(pub Handle<LoadedFolder>);

#[derive(Resource)]
pub struct AssetFoldersLeftToLoad(pub HashSet<AssetId<LoadedFolder>>);
