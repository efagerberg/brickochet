use bevy::prelude::*;
use bevy::{diagnostic, window};
use bevy_inspector_egui::{bevy_egui, quick};

mod components;
mod resources;
mod systems;

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: window::PresentMode::Immediate, // 🚫 VSync OFF
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            diagnostic::FrameTimeDiagnosticsPlugin::default(),
            diagnostic::LogDiagnosticsPlugin::default(),
        ))
        .add_plugins(bevy_egui::EguiPlugin::default())
        .add_plugins(quick::WorldInspectorPlugin::default())
        .insert_resource(resources::playfield::Playfield {
            half_width: 20.0,
            half_height: 15.0,
            half_depth: 30.0,
        })
        .add_systems(Startup, systems::scene::setup)
        .add_systems(
            Update,
            (
                systems::input::grab_mouse,
                systems::paddle::paddle_mouse_control,
                systems::ball::reflect_ball,
                systems::ball::move_ball,
                systems::paddle::paddle_ball_collision,
            ),
        )
        .run();
}
