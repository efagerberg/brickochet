use bevy::{asset::LoadedFolder, prelude::*};

#[derive(Default, Resource)]
pub struct BrickFolder(pub Handle<LoadedFolder>);
