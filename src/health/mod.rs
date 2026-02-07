use bevy::prelude::*;

use crate::states;

pub mod components;
pub mod messages;
pub mod systems;

#[cfg(test)]
mod tests;

pub fn plugin(app: &mut App) {
    app.add_message::<messages::HealthChangedMessage>()
        .add_message::<messages::DeathMessage>()
        .add_systems(
            Update,
            (systems::handle_health_changed, systems::handle_death)
                .run_if(in_state(states::GameState::Gameplay)),
        )
        .add_systems(
            FixedUpdate,
            systems::handle_collision
                .after(crate::physics::PhysicsSet::ResolveCollisions)
                .run_if(in_state(states::GameState::Gameplay)),
        )
        .add_systems(
            PostUpdate,
            systems::update_health_color
                .before(crate::rendering::RenderingSet::Integrate)
                .run_if(in_state(states::GameState::Gameplay)),
        );
}
