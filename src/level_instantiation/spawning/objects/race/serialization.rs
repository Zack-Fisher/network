use bevy::{prelude::*, tasks::Task, utils::HashMap};

use bevy_pkv::{PkvStore, SetError};
use serde::{Serialize, Deserialize};
use strum::IntoEnumIterator;

use crate::level_instantiation::spawning::objects::race::Races;

use super::{RaceTable, RaceData};

//avoid typos with const slices.
//these basically work as C macros.
const RACETABLE_KEY: &str = "racetable";

pub struct RaceSerializationPlugin;

impl Plugin for RaceSerializationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<RaceTableSave>()
            .add_event::<RaceTableLoad>()
            .add_event::<RaceTableInit>()

            .add_system(save_racetable)
            .add_system(load_racetable)
            .add_system(init_racetable)
            ;
    }
}

pub struct RaceTableSave;

// #[derive(Component)]
// struct RaceSaveTask(Task<Result<(), SetError>>);

//poll the event line for a save request.
fn save_racetable (
    mut pkv: ResMut<PkvStore>,
    table: Res<RaceTable>,

    mut save_evr: EventReader<RaceTableSave>,
)
{
    for _ in save_evr.iter() {
        pkv.set(RACETABLE_KEY, &table.table.clone());
    }
}

pub struct RaceTableLoad;

fn load_racetable (
    pkv: ResMut<PkvStore>,
    mut load_evr: EventReader<RaceTableLoad>,

    mut init_evw: EventWriter<RaceTableInit>,

    mut table: ResMut<RaceTable>,
)
{
    for _ in load_evr.iter() {
        if let Ok(load_table) = pkv.get::<HashMap<String, RaceData>>(RACETABLE_KEY) {
            info!("found the racetable in memory, loading into the resource...");
            table.table = load_table;
        } else {
            info!("cannot load racetable, it's not found in the pkv store. sending raceinit signal...");
            init_evw.send(RaceTableInit);
        }
    }
}

pub struct RaceTableInit;

/// initialize a racetable if it isn't already found.
fn init_racetable (
    mut init_evr: EventReader<RaceTableInit>,

    mut save_evw: EventWriter<RaceTableSave>,

    mut racetable: ResMut<RaceTable>,
)
{
    for _ in init_evr.iter() {
        info!("racetable not found, initializing...");

        //initialize from the pkv racetable storage.
        for race_value in Races::iter() {
            racetable.table.insert(format!("{:?}", race_value).clone(), RaceData::default());
        }

        info!("initialized racetable. saving to system memory...");
        save_evw.send(RaceTableSave);
    }
}