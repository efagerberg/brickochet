use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuButtonAction {
    Play,
    Quit,
}

#[derive(Component)]
pub struct SelectedOption;


pub type MenuInteraction<'a> = (&'a Interaction, &'a MenuButtonAction);
pub type MenuButtonInteraction<'a> = (
    &'a Interaction,
    &'a mut BackgroundColor,
    Option<&'a SelectedOption>,
);
pub type RecentButtonInteraction = (Changed<Interaction>, With<Button>);