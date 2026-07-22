use bevy::prelude::*;
use bevy::window;
use bevy_common_assets::ron;
use bevy_inspector_egui::bevy_egui;

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
    app.add_plugins((
        bevy_embedded_assets::EmbeddedAssetPlugin {
            mode: bevy_embedded_assets::PluginMode::ReplaceDefault,
        },
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    // https://github.com/bevyengine/bevy/issues/3317
                    present_mode: window::PresentMode::Mailbox, // 🚫 VSync OFF
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin::default()),
        bevy_egui::EguiPlugin::default(),
        ron::RonAssetPlugin::<brick::assets::BrickAsset>::new(&["brick.ron"]),
        bevy_framepace::FramepacePlugin,
    ))
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
        use bevy::diagnostic;
        use bevy_inspector_egui::quick;
        app.add_plugins((
            quick::WorldInspectorPlugin::default(),
            diagnostic::LogDiagnosticsPlugin::default(),
            diagnostic::FrameTimeDiagnosticsPlugin::default(),
        ));
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
