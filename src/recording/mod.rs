use std::collections::VecDeque;

use bevy::{prelude::*, utils::HashMap};
use bevy_pkv::PkvStore;
use leafwing_input_manager::prelude::ActionState;

use crate::player_control::{actions::PlayerAction, player_embodiment::Player};

pub mod input;

pub struct RecordingPlugin;

impl Plugin for RecordingPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(RecordingTable {
                table: HashMap::new(),
                curr_recording: None,
            })

            .add_event::<SaveRecordingTable>()
            .add_event::<InitRecordingTable>()

            .add_system(save_table_process)
            .add_system(init_table_process)

            .add_startup_system(table_initter)

            .add_system(current_recording_manager)

            .add_plugin(input::ActionInputPlugin)
            .add_system(record_player_actions)
            ;
    }
}

fn record_player_actions (
    mut player_action_q: Query<&ActionState<PlayerAction>, With<Player>>,

    mut local_table: ResMut<RecordingTable>,
)
{
    match local_table.curr_recording.clone() {
        Some(recording_name) => {
            if let Some(action_vec) = local_table.table.get_mut(&recording_name.clone()) {
                for player_actions in player_action_q.iter_mut() {
                    action_vec.push_back(player_actions.clone());
                }
            } else {
                //if the current recordingname isn't present, make a new one and start recording in it.
                info!("recording not found! creating new recordingtable elm at {}", recording_name.clone());
                local_table.table.insert(recording_name.clone(), VecDeque::new());
            }
        }
        None => {
            //we're not recording right now.
        }
    }
}

fn current_recording_manager (
    mut local_table: ResMut<RecordingTable>,
)
{
    local_table.curr_recording = Some("john".to_string());
}

pub struct SaveRecordingTable;

pub struct InitRecordingTable;

const RECORDINGTABLE_KEY: &str = "recordingtable";

#[derive(Resource)]
pub struct RecordingTable {
    pub table: HashMap<String, VecDeque<ActionState<PlayerAction>>>,

    /// this is the member of the table that it's currently recording all player actions to.
    /// we're not always recording, so make this an option.
    pub curr_recording: Option<String>,
}

fn save_table_process (
    mut pkv: ResMut<PkvStore>,
    local_table: ResMut<RecordingTable>,

    mut save_evr: EventReader<SaveRecordingTable>,
)
{
    for _ in save_evr.iter() {
        info!("saving the recordingtable to memory...");
        pkv.set::<HashMap<String, VecDeque<ActionState<PlayerAction>>>>(RECORDINGTABLE_KEY, &local_table.table.clone());
    }
}

fn init_table_process (
    mut pkv: ResMut<PkvStore>,
    mut local_table: ResMut<RecordingTable>,
    mut save_evw: EventWriter<SaveRecordingTable>,

    mut init_evr: EventReader<InitRecordingTable>,
)
{
    for _ in init_evr.iter() {
        if let Ok(saved_table) = pkv.get::<HashMap<String, VecDeque<ActionState<PlayerAction>>>>(RECORDINGTABLE_KEY) {
            info!("found the recordingtable saved from memory. loading it into the app...");

            local_table.table = saved_table.clone();
        } else {
            info!("couldn't find a recordingtable in memory. making a new one...");
            local_table.table = HashMap::new();

            info!("created recordingtable! saving to memory...");
            save_evw.send(SaveRecordingTable);
        }
    }
}

fn table_initter (
    mut init_evw: EventWriter<InitRecordingTable>,
)
{
    init_evw.send(InitRecordingTable);
}
