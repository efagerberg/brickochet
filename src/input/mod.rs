use bevy::prelude::*;

use crate::states;

pub mod systems;

#[cfg(test)]
mod tests;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(states::GameState::Gameplay), systems::grab_cursor);
    app.add_systems(
        Update,
        systems::update_cursor_on_mouse_input.run_if(in_state(states::GameState::Gameplay)),
    );
}
