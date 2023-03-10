use std::collections::VecDeque;

use bevy::{prelude::*, utils::HashMap};
use leafwing_input_manager::prelude::ActionState;

use crate::player_control::{actions::PlayerAction, player_embodiment::Player};

pub mod input;

pub struct RecordingPlugin;

impl Plugin for RecordingPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CurrentRecording {
                actions: vec![],
            })

            .add_event::<SaveRecording>()

            .add_plugin(input::ActionInputPlugin)
            .add_system(record_player_actions)
            ;
    }
}

use serde::{Serialize, Deserialize};

//dump all input into this one vector, decide seperately whether to dump it into memory.
//be careful not to overload this vector.
#[derive(Default, Component, Reflect, Serialize, Deserialize, Resource)]
#[reflect(Component, Serialize, Deserialize)]
pub struct CurrentRecording {
    actions: Vec<ActionState<PlayerAction>>,
}

//write to localstorage with wasm bevy_pkv if using the web build

//write to disk if not?
//should this all be the same file? probably not.
//either way, make writing its own seperate thing, since we have to do two kinds of writing

fn record_player_actions (
    mut player_action_q: Query<&ActionState<PlayerAction>, With<Player>>,

    mut curr_recording: ResMut<CurrentRecording>,

    time: Res<Time>,
)
{
    for player_actions in player_action_q.iter_mut() {
        curr_recording.actions.push(player_actions.clone());       
    }
}

pub struct SaveRecording {

}

fn save_to_storage (
    mut curr_recording: ResMut<CurrentRecording>,

    mut save_evr: EventReader<SaveRecording>,
)
{
    for ev in save_evr.iter() {
        let mut deque: VecDeque<ActionState<PlayerAction>> = VecDeque::new();

        deque.extend(curr_recording.actions.clone());
        
        //TODO: implement writing this to disk with wasm and filesys

        //clear out the current actions list.
        curr_recording.actions = vec![];
    }
}
