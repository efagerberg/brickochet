use bevy::prelude::*;
use bevy::window;
use bevy_common_assets::ron;
use bevy_inspector_egui::{bevy_egui, quick};

use crate::gameplay::brick;

mod asset_loading;
mod audio;
mod gameplay;
mod health;
mod input;
mod main_menu;
mod physics;
mod rendering;
mod scene;
mod states;

#[cfg(test)]
mod test_utils;

fn main() -> Result<(), BevyError> {
    let mut app = App::new();
    // Workaround for https://github.com/bevyengine/bevy/issues/22103
    let asset_path =
        std::env::current_dir().map(|p| p.join("assets").to_string_lossy().into_owned())?;
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    // https://github.com/bevyengine/bevy/issues/3317
                    present_mode: window::PresentMode::Immediate, // 🚫 VSync OFF
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                file_path: asset_path,
                ..default()
            }),
    )
    .add_plugins(bevy_egui::EguiPlugin::default())
    .add_plugins(ron::RonAssetPlugin::<brick::assets::BrickAsset>::new(&[
        "brick.ron",
    ]))
    .add_plugins((
        audio::plugin,
        states::plugin,
        asset_loading::plugin,
        scene::plugin,
        gameplay::plugin,
        physics::plugin,
        rendering::plugin,
        health::plugin,
        main_menu::plugin,
        input::plugin,
    ))
    .add_systems(Startup, setup_egui_settings);

    #[cfg(debug_assertions)]
    {
        app.add_plugins(quick::WorldInspectorPlugin::default());
    }

    app.run();
    Ok(())
}

fn setup_egui_settings(
    mut egui_settings: ResMut<bevy_inspector_egui::bevy_egui::EguiGlobalSettings>,
) {
    // Instead of attaching to first spawned camera, manually set up the context
    egui_settings.auto_create_primary_context = false;
}
