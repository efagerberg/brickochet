use bevy::prelude::*;

use crate::states;

pub mod components;
pub mod systems;

pub fn plugin(app: &mut App) {
    app.add_systems(
        PostUpdate,
        systems::play_collide_sfx.run_if(in_state(states::GameState::Gameplay)),
    );
}
