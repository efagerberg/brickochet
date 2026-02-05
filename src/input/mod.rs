use bevy::prelude::*;

pub mod systems;

#[cfg(test)]
mod tests;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, systems::grab_mouse);
}
