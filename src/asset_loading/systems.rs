use std::collections::HashSet;

use crate::{asset_loading, states};
use bevy::{asset::LoadedFolder, prelude::*};

pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let folders_to_load = vec!["bricks/", "audio/", "textures/"];
    let mut asset_ids: HashSet<AssetId<LoadedFolder>> = HashSet::new();

    for path in folders_to_load {
        let handles = asset_server.load_folder(path);
        asset_ids.insert(handles.id());

        match path {
            "bricks/" => {
                commands.insert_resource(asset_loading::resources::LoadedBrickFolder(handles));
            }
            "audio/" => {
                commands.insert_resource(asset_loading::resources::LoadedAudioFolder(handles));
            }
            "textures/" => {
                commands.insert_resource(asset_loading::resources::LoadedTextureFolder(handles));
            }
            _ => {}
        }
    }

    commands.insert_resource(asset_loading::resources::AssetFoldersLeftToLoad(asset_ids));
}

pub fn check_assets(
    mut app_next_state: ResMut<NextState<states::GameState>>,
    mut messages: MessageReader<AssetEvent<LoadedFolder>>,
    mut asset_folders_left_to_load: ResMut<asset_loading::resources::AssetFoldersLeftToLoad>,
) {
    for event in messages.read() {
        if let AssetEvent::LoadedWithDependencies { id } = event {
            asset_folders_left_to_load.0.remove(id);
        }
    }
    if asset_folders_left_to_load.0.is_empty() {
        app_next_state.set(states::GameState::Menu);
    }
}
