use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::player_control::{actions::PlayerAction, player_embodiment::Player, action_handler::ActionStream};

pub struct ActionInputPlugin;

impl Plugin for ActionInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(handle_player_input)
            ;
    }
}

fn handle_player_input (
    mut player_q: Query<(&ActionState<PlayerAction>, &mut ActionStream), With<Player>>,
)
{
    //don't reinvent the wheel, just use the same exact type as before.
    for (player_action, mut action_stream) in player_q.iter_mut() {
        action_stream.actions.push_back(player_action.clone());
    }
}
