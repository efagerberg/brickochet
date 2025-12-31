use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui, quick};
use bevy_rapier3d::prelude::*;

mod resources;
mod systems;
mod components;

fn main() {
    App::new()
        .insert_resource(resources::input::MousePosition::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(bevy_egui::EguiPlugin::default())
        .add_plugins(quick::WorldInspectorPlugin::default())
        .add_systems(
            Startup,
            systems::scene::setup,
        )
        .add_systems(Update, (systems::input::update_mouse_position, systems::paddle::paddle_mouse_control))
        .run();
}
