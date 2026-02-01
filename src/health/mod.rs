use bevy::prelude::*;

pub mod components;
pub mod messages;
pub mod systems;

#[cfg(test)]
mod tests;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<messages::HealChangedMessage>()
            .add_message::<messages::DeathMessage>()
            .add_systems(
                Update,
                (systems::handle_health_changed, systems::handle_death),
            )
            .add_systems(
                PostUpdate,
                (systems::update_health_color,).before(crate::rendering::RenderingSet::Integrate),
            );
    }
}
