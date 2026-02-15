use bevy::prelude::*;

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    LoadingAssets,
    Menu,
    Gameplay,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    #[default]
    Disabled,
    Main,
}

pub fn plugin(app: &mut App) {
    app.init_state::<GameState>();
    app.init_state::<MenuState>();
}
