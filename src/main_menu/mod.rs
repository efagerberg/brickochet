use bevy::prelude::*;

use crate::states;

#[cfg(test)]
mod tests;

pub mod components;
pub mod systems;

pub fn plugin(app: &mut App) {
    app.init_state::<states::MenuState>()
        .add_systems(OnEnter(states::GameState::Menu), systems::menu_setup)
        .add_systems(
            Update,
            (systems::menu_action, systems::button_system)
                .run_if(in_state(states::GameState::Menu)),
        )
        .add_systems(OnEnter(states::MenuState::Main), systems::menu_ui_setup);
}
