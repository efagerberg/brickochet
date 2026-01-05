use bevy::prelude::*;
use test_case::test_case;

use crate::input;

#[derive(Default)]
struct GrabMouseCase {
    cursor_options: bevy::window::CursorOptions,
    press_mouse: Option<MouseButton>,
    press_key: Option<KeyCode>,
    expected_grab_mode: bevy::window::CursorGrabMode,
    expected_visible: bool,
}

#[test_case(
    GrabMouseCase {
        cursor_options: bevy::window::CursorOptions {
            visible: true,
            grab_mode: bevy::window::CursorGrabMode::None,
            ..default()
        },
        press_mouse: Some(MouseButton::Left),
        press_key: None,
        expected_grab_mode: bevy::window::CursorGrabMode::Locked,
        expected_visible: false
    }
)]
#[test_case(
    GrabMouseCase {
        cursor_options: bevy::window::CursorOptions {
            visible: false,
            grab_mode: bevy::window::CursorGrabMode::Locked,
            ..default()
        },
        press_mouse: None,
        press_key: Some(KeyCode::Escape),
        expected_grab_mode: bevy::window::CursorGrabMode::None,
        expected_visible: true
    }
)]
fn test_grab_mouse_alters_cursor_options_given_expected_input(case: GrabMouseCase) {
    let mut app = App::new();
    app.add_systems(Update, input::systems::grab_mouse);
    let entity = app.world_mut().spawn(
        case.cursor_options
    ).id();
    let mut keyboard_input = ButtonInput::<KeyCode>::default();
    if let Some(press_key) = case.press_key {
        keyboard_input.press(press_key);
    }
    app.insert_resource(keyboard_input);

    let mut mouse_input: ButtonInput<MouseButton> = ButtonInput::<MouseButton>::default();
    if let Some(press_mouse) = case.press_mouse {
        mouse_input.press(press_mouse);
    }
    app.insert_resource(mouse_input);

    app.update();
    let updated_cursor_options = app.world().get::<bevy::window::CursorOptions>(entity).unwrap();

    assert_eq!(updated_cursor_options.grab_mode, case.expected_grab_mode);
    assert_eq!(updated_cursor_options.visible, case.expected_visible);
}