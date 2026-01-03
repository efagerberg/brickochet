use bevy::prelude::*;
use bevy::{diagnostic, window};
use bevy_inspector_egui::{bevy_egui, quick};

mod components;
mod resources;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: window::PresentMode::Immediate, // 🚫 VSync OFF
                ..default()
            }),
            ..default()
        }))
        // .add_plugins((
        //     diagnostic::FrameTimeDiagnosticsPlugin::default(),
        //     diagnostic::LogDiagnosticsPlugin::default(),
        // ))
        .add_plugins(bevy_egui::EguiPlugin::default())
        .add_plugins(quick::WorldInspectorPlugin::default())
        .add_systems(Startup, systems::scene::setup)
        .add_systems(
            Update,
            (
                systems::input::grab_mouse,
                systems::paddle::paddle_mouse_control,
                (
                    systems::physics::apply_curve,
                    systems::physics::apply_velocity,
                )
                    .chain(),
                (
                    systems::paddle::record_paddle_motion,
                    systems::paddle::paddle_ball_collision,
                    systems::ball::reflect_ball,
                    systems::paddle::apply_curve_from_motion_record,
                    systems::playfield::highlight_depth_lines
                )
                    .chain(),
            ),
        )
        .run();
}
