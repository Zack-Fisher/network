use bevy::{prelude::*, utils::HashMap};
use leafwing_input_manager::prelude::ActionState;

use crate::player_control::actions::PlayerAction;

pub struct RecordingPlugin;

impl Plugin for RecordingPlugin {
    fn build(&self, app: &mut App) {
        app
            ;
    }
}

//we load recordings into the table and pull from them with ghosts at runtime.
pub struct RecordingTable {
    table: HashMap<String, PlayerRecording>,
}

pub struct PlayerRecording {
    actions: Vec<PlayerAction>,
}

//write to localstorage with wasm bevy_pkv if using the web build

//write to disk if not?
//should this all be the same file? probably not.
//either way, make writing its own seperate thing, since we have to do two kinds of writing

fn record_player_actions (
    mut player_action_q: Query<(&ActionState<PlayerAction>)>,

    time: Res<Time>,
)
{
    for player_actions in player_action_q.iter_mut() {
        
    }
}