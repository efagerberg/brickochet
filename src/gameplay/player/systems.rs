use bevy::prelude::*;

use crate::gameplay::player;
use crate::{health, states};

pub fn restart_on_player_death(
    player_query: Query<Entity, With<player::components::Player>>,
    mut death_messages: MessageReader<health::messages::DeathMessage>,
    mut game_state: ResMut<NextState<states::GameState>>,
) {
    for message in death_messages.read() {
        if player_query.contains(message.entity) {
            game_state.set(states::GameState::Menu);
        }
    }
}
