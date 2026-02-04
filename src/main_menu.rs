use bevy::color::palettes::css::CRIMSON;
use bevy::prelude::*;

use crate::state;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    #[default]
    Disabled,
}

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Quit,
}

#[derive(Component)]
pub struct SelectedOption;

type MenuInteraction<'a> = (&'a Interaction, &'a MenuButtonAction);
type MenuButtonInteraction<'a> = (
    &'a Interaction,
    &'a mut BackgroundColor,
    Option<&'a SelectedOption>,
);
type RecentButtonInteraction = (Changed<Interaction>, With<Button>);

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn button_system(mut interaction_query: Query<MenuButtonInteraction, RecentButtonInteraction>) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

pub fn menu_setup(mut commands: Commands, mut menu_state: ResMut<NextState<MenuState>>) {
    commands.spawn((
        Camera2d,
        bevy_inspector_egui::bevy_egui::PrimaryEguiContext,
        DespawnOnExit(state::GameState::Menu),
    ));
    menu_state.set(MenuState::Main);
}

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

pub fn menu_ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Common style for all buttons on the screen
    let button_node = Node {
        width: px(300),
        height: px(65),
        margin: UiRect::all(px(20)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_node = Node {
        width: px(30),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: px(10),
        ..default()
    };
    let button_text_font = TextFont {
        font_size: 33.0,
        ..default()
    };

    let right_icon = asset_server.load("textures/icons/right.png");
    let exit_icon = asset_server.load("textures/icons/exitRight.png");

    commands.spawn((
        DespawnOnExit(MenuState::Main),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            children![
                // Display the game name
                (
                    Text::new("Brickochet"),
                    TextFont {
                        font_size: 67.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    Node {
                        margin: UiRect::all(px(50)),
                        ..default()
                    },
                ),
                // Display two buttons for each action available from the main menu:
                // - new game
                // - quit
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Play,
                    children![
                        (ImageNode::new(right_icon), button_icon_node.clone()),
                        (
                            Text::new("New Game"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ),
                    ]
                ),
                (
                    Button,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Quit,
                    children![
                        (ImageNode::new(exit_icon), button_icon_node),
                        (Text::new("Quit"), button_text_font, TextColor(TEXT_COLOR),),
                    ]
                ),
            ]
        )],
    ));
}

pub fn menu_action(
    interaction_query: Query<MenuInteraction, RecentButtonInteraction>,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<state::GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_writer.write(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    game_state.set(state::GameState::Gameplay);
                    menu_state.set(MenuState::Disabled);
                }
            }
        }
    }
}

pub fn plugin(app: &mut App) {
    app.init_state::<MenuState>()
        .add_systems(OnEnter(state::GameState::Menu), menu_setup)
        .add_systems(
            Update,
            (menu_action, button_system).run_if(in_state(state::GameState::Menu)),
        )
        .add_systems(OnEnter(MenuState::Main), menu_ui_setup);
}
