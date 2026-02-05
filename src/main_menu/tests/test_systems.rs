use bevy::prelude::*;
use test_case::test_case;

use crate::main_menu::{components, systems};
use crate::states;
use crate::test_utils;

/// A struct representing a single test case for the button system
#[derive(Debug)]
struct ButtonTestCase {
    interaction: Interaction,
    selected: bool,
    expected_color: Color,
}

/// Sets up a minimal Bevy app with the button system and spawns a test button.
/// Returns the app and the spawned entity.
fn setup_button_test(interaction: Interaction, selected: bool) -> (App, Entity) {
    let mut app = App::new();

    app.add_plugins((
        MinimalPlugins,
        bevy::state::app::StatesPlugin,
        states::plugin,
    ));
    app.add_systems(Update, systems::button_system);

    let entity = app
        .world_mut()
        .spawn(Button)
        .insert(BackgroundColor(systems::NORMAL_BUTTON))
        .insert(components::MenuButtonAction::Play)
        .insert(interaction)
        .id();

    if selected {
        app.world_mut()
            .entity_mut(entity)
            .insert(components::SelectedOption);
    }

    (app, entity)
}

#[test_case(ButtonTestCase { interaction: Interaction::None, selected: false, expected_color: systems::NORMAL_BUTTON } ; "none + not selected")]
#[test_case(ButtonTestCase { interaction: Interaction::Hovered, selected: false, expected_color: systems::HOVERED_BUTTON } ; "hovered + not selected")]
#[test_case(ButtonTestCase { interaction: Interaction::Hovered, selected: true, expected_color: systems::HOVERED_PRESSED_BUTTON } ; "hovered + selected")]
#[test_case(ButtonTestCase { interaction: Interaction::Pressed, selected: false, expected_color: systems::PRESSED_BUTTON } ; "pressed + not selected")]
#[test_case(ButtonTestCase { interaction: Interaction::None, selected: true, expected_color: systems::PRESSED_BUTTON } ; "none + selected")]
fn test_button_system_struct_cases(case: ButtonTestCase) {
    let (mut app, entity) = setup_button_test(case.interaction, case.selected);
    app.update();

    let color = app.world().get::<BackgroundColor>(entity).unwrap().0;
    assert_eq!(color, case.expected_color);
}

#[derive(Debug)]
struct MenuActionTestCase {
    action: components::MenuButtonAction,
    interaction: Interaction,
    should_exit: bool,
    expected_game_state: states::GameState,
    expected_menu_state: states::MenuState,
}

fn setup_menu_action_test(case: &MenuActionTestCase) -> (App, Entity) {
    let mut app = App::new();

    app.add_plugins((
        MinimalPlugins,
        bevy::state::app::StatesPlugin,
        states::plugin,
    ));
    app.add_systems(Update, systems::menu_action);

    let entity = app
        .world_mut()
        .spawn((Button, case.action, case.interaction))
        .id();

    (app, entity)
}

#[test_case(
    MenuActionTestCase {
        action: components::MenuButtonAction::Play,
        interaction: Interaction::Pressed,
        should_exit: false,
        expected_game_state: states::GameState::Gameplay,
        expected_menu_state: states::MenuState::Disabled,
    }
; "when play button pressed, moves to gameplay state")]
#[test_case(
    MenuActionTestCase {
        action: components::MenuButtonAction::Quit,
        interaction: Interaction::Pressed,
        should_exit: true,
        expected_game_state: states::GameState::Menu, // unchanged
        expected_menu_state: states::MenuState::Disabled,    // unchanged
    }
; "when quit button pressed, exits app")]
#[test_case(
    MenuActionTestCase {
        action: components::MenuButtonAction::Play,
        interaction: Interaction::Hovered,
        should_exit: false,
        expected_game_state: states::GameState::Menu, // unchanged
        expected_menu_state: states::MenuState::Disabled, // unchanged
    }
; "wen play button hovered, no state change but color changes")]
fn test_menu_action_system(case: MenuActionTestCase) {
    let (mut app, _entity) = setup_menu_action_test(&case);
    // Update once for the processing of the system and once for the state change to
    // take effect
    app.update();
    app.update();

    let game_state = app.world().resource::<State<states::GameState>>();
    let menu_state = app.world().resource::<State<states::MenuState>>();

    assert_eq!(*game_state, case.expected_game_state);
    assert_eq!(*menu_state, case.expected_menu_state);
    let expected_message_count = if case.should_exit { 1 } else { 0 };
    test_utils::assertions::assert_message_count::<AppExit>(&app, expected_message_count);
}
