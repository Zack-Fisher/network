use bevy::{prelude::*, utils::HashMap};
use bevy_pkv::PkvStore;
use strum::IntoEnumIterator;

use crate::GameState;

use serde::{Serialize, Deserialize};

pub mod checkpoint;
pub mod serialization;

use checkpoint::*;

use self::serialization::{RaceTableSave, RaceTableLoad};

pub struct RacePlugin;

impl Plugin for RacePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(RaceState::Paused)

            .add_plugin(serialization::RaceSerializationPlugin)

            .add_event::<FlushCheckpointCounts>()
            .add_event::<ActivateRace>()

            .insert_resource(
                RaceTable {
                    table: HashMap::new(),
                    active_race: "NULL".to_string(),
                    is_racing: false,
                }
            )

            .add_startup_system(racetable_init)

            .add_system(racetable_process)
            .add_system(race_process)

            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(checkpoint::build_checkpoint)
                    .with_system(checkpoint::checkpoint_process)
            )
            ;
    }
}

//all of the races and checkpoints are identified by eachother with a unique name string.
//we'll pull this out into a enum to make things easier in design.
//we'll also reduce the amount of work we're doing per race with a cool trick.
//if we derive debug, we can automatically convert any variant to a stable string.
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, strum_macros::EnumIter, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub enum Races {
    #[default]
    TestRace,
    BetaRace,
}

//the RaceState is polled for by things like the racetimer.
#[derive(Debug, Clone, Eq, PartialEq, Hash)] 
pub enum RaceState { 
    Paused,
    InRace,
}

//PUT EVERYTHING IN THE SAME DATA STRUCTURE.
//STOP SUCKING AT RUST
//OH MY GOD
#[derive(Resource, Debug, Serialize, Deserialize)]
pub struct RaceTable {
    pub table: HashMap<String, RaceData>,
    pub active_race: String,
    pub is_racing: bool,
}

/// just sends an event to the appropriate module.
/// this will attempt to load from memory, then initialize a new racetable if it's not found.
/// then, the initted table will be loaded to the resource and saved to memory.
fn racetable_init (
    mut loadtable_evw: EventWriter<RaceTableLoad>,
)
{
    loadtable_evw.send(RaceTableLoad);
}

//the racetable should poll the World and manage itself accordingly.
//please make sure that only one RaceData element in the mapping is considered "active" at any given time.
//the racedata should hold highscores and stuff, and it should be serializable.
//we already have the hashmapping at loadtime, from the racetable_init method.
fn racetable_process (
    mut racetable: ResMut<RaceTable>,    

    mut check_q: Query<&CheckpointPrefab, Added<CheckpointPrefab>>,
)
{
    match racetable.table.get(&racetable.active_race.clone()) {
        Some(race_data) => {
            //if the currently active race is inactive, make the whole table inactive.
            if !race_data.is_active {
                racetable.is_racing = false;
            } else {
                racetable.is_racing = true;
            }
        }
        None => {

        }
    }
}

//an event line.
//probably should be called on a WorldLoadRequest, should this even be seperate?
pub struct FlushCheckpointCounts;

//the checkpoints call this one.
//each checkpoint emits this signal onto the line with their race_name
//they can't set it directly because we need to do a bunch of micromanagement stuff to make this work.
pub struct ActivateRace {
    pub name: String,
}

fn race_process (
    mut racetable: ResMut<RaceTable>,

    check_q: Query<&CheckpointPrefab, Added<CheckpointPrefab>>,

    mut flush_evr: EventReader<FlushCheckpointCounts>,
    mut activate_evr: EventReader<ActivateRace>,

    mut racesave_evw: EventWriter<RaceTableSave>,

    time: Res<Time>,
)
{
    for ev in flush_evr.iter() {
        racetable.table.iter_mut().for_each(|(_, value)| {
            value.checkpoint_count = 0;
            value.hit_count = 0;
        });
        info!("flushing the racetable! FlushCheckpointCounts signal recieved!");
    }
    
    //remember, CheckpointPrefab has the Added condition in the query, so this should work fine.
    for check_prefab in check_q.iter() {
        match racetable.table.get_mut(&format!("{:?}", check_prefab.race_name)) {
            Some(value) => {
                value.checkpoint_count += 1;
            },
            None => {
                info!("the RaceData '{:?}' was not found in the hashtable!! ahh!!", check_prefab.race_name);
                continue;
            }
        };
    }

    for ev in activate_evr.iter() {
        //if it's already activated in the hashmap, continue.
        //otherwise, activate it and flush all the other races.

        match racetable.table.get(&ev.name.clone()) {
            Some(race_data) => {
                if race_data.is_active {
                    continue;
                }
            }
            None => {

            }
        }

        let mut key_string = String::from("");
        
        racetable.table.iter_mut().for_each(|(key, value_iter)| {
            //keys are unique (?) so this guarantees only one can be active at a time.

            //this is AFTER the activation bump, so the hit count will already be one.
            value_iter.hit_count = 1;
            if ev.name.clone() == key.clone() {
                value_iter.is_active = true;

                key_string = key.clone();

                info!("freshly activated this race: {:?}", value_iter);
            } else {
                value_iter.is_active = false;
                //reset the hit count to zero to make management of child checkpoints remotely easier.
            }
        });

        racetable.active_race = key_string;
    }

    //no point in doing checks if there's no active race.
    if !racetable.is_racing {
        return;
    }

    //now, process the current race.

    //cloning the string out on a seperate line prevents an immutable borrow on top of the mutable borrow.
    let active_race = racetable.active_race.clone();
    if let Some(race_data) = racetable.table.get_mut(&active_race) {
        race_data.curr_time += time.delta_seconds();

        if race_data.hit_count >= race_data.checkpoint_count {
            //THE RACE IS FINISHED
            info!("finished race");
            match race_data.best_time {
                Some(time) => {
                    if time >= race_data.curr_time {
                        info!("new best time on race: {}", active_race.clone());
                        info!("old time was {}, new best is {}", time, race_data.curr_time);

                        race_data.best_time = Some(race_data.curr_time);
                    } else {
                        info!("no new best time.");
                    }
                }
                None => {
                    race_data.best_time = Some(race_data.curr_time);
                }
            }
            race_data.curr_time = 0.0;
            race_data.amount_finished += 1;
            race_data.is_active = false;

            //save to localstorage each time a race has finished.
            racesave_evw.send(RaceTableSave);
        }
    }
}

//not a component, just exists in the hashtable
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct RaceData {
    pub is_active: bool,
    pub curr_time: f32,
    pub best_time: Option<f32>,
    //the total checkpoints associated with this race.
    pub checkpoint_count: u32,
    //the amount of checkpoints of this particular race that have been hit.
    pub hit_count: u32,
    //the amount of times the race has been completed.
    pub amount_finished: u32,
}