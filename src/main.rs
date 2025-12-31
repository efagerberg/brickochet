use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui, quick};
use bevy_rapier3d::prelude::*;

mod components;
mod resources;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(bevy_egui::EguiPlugin::default())
        .add_plugins(quick::WorldInspectorPlugin::default())
        .add_systems(
            Startup,
            systems::scene::setup,
        )
        .run();
}
