use bevy::prelude::*;

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Gameplay,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    #[default]
    Disabled,
}

pub fn plugin(app: &mut App) {
    app.init_state::<GameState>();
    app.init_state::<MenuState>();
}
