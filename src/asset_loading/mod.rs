use bevy::prelude::*;

use crate::states;

pub mod resources;
pub mod systems;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(states::GameState::LoadingAssets),
        systems::load_assets,
    )
    .add_systems(
        Update,
        systems::check_assets.run_if(in_state(states::GameState::LoadingAssets)),
    );
}
