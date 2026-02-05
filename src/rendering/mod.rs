use bevy::prelude::*;

pub mod messages;
pub mod systems;

#[cfg(test)]
mod tests;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum RenderingSet {
    Integrate,
}

pub fn plugin(app: &mut App) {
    app.add_message::<messages::MaterialColorsChangedMessage>()
        .configure_sets(PostUpdate, RenderingSet::Integrate)
        .add_systems(
            PostUpdate,
            systems::update_material_color.in_set(RenderingSet::Integrate),
        );
}
