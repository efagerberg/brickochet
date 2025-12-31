use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui, quick};
use bevy_rapier3d::prelude::*;
use bevy::{diagnostic, window};

mod resources;
mod systems;
mod components;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: window::PresentMode::Immediate, // 🚫 VSync OFF
                    ..default()
                }),
                ..default()
            }))
        .add_plugins((diagnostic::FrameTimeDiagnosticsPlugin::default(), diagnostic::LogDiagnosticsPlugin::default()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(bevy_egui::EguiPlugin::default())
        .add_plugins(quick::WorldInspectorPlugin::default())
        .add_systems(Startup, systems::scene::setup)
        .add_systems(Update, systems::paddle::paddle_mouse_control)
        .run();
}
