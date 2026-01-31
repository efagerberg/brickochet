use bevy::prelude::*;
use bevy::window;
use bevy_inspector_egui::{bevy_egui, quick};

mod gameplay;
mod health;
mod input;
mod physics;
mod rendering;
mod scene;

#[cfg(test)]
mod test_utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // https://github.com/bevyengine/bevy/issues/3317
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
        .add_plugins((
            scene::ScenePlugin,
            gameplay::GameplayPlugin,
            physics::PhysicsPlugin,
            rendering::RenderingPlugin,
            health::HealthPlugin,
        ))
        .add_systems(Update, input::systems::grab_mouse)
        .run();
}
