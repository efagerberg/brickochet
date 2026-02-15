use crate::states;
use bevy::{asset::LoadedFolder, prelude::*};

pub fn check_assets(
    mut app_next_state: ResMut<NextState<states::GameState>>,
    mut messages: MessageReader<AssetEvent<LoadedFolder>>,
) {
    for event in messages.read() {
        if let AssetEvent::LoadedWithDependencies { id: _ } = event {
            app_next_state.set(states::GameState::Menu);
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        check_assets.run_if(in_state(states::GameState::LoadingAssets)),
    );
}
