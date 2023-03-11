// a programmer should be able to pull and modify the Flags resource from anywhere.
// i don't think we need an event interfacing system. that's probably overkill.
use bevy::{prelude::*, utils::HashMap};

use bevy_pkv::PkvStore;

pub struct FlagPlugin;

impl Plugin for FlagPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(
                FlagsTable::default()
            )

            .add_event::<SaveFlags>()
            .add_event::<InitFlags>()

            .add_system(init_flag_process)
            .add_system(save_flag_process)

            .add_startup_system(flag_initter)
            ;
    }
}

//a wrapper struct
#[derive(Resource, Default)]
pub struct FlagsTable{
    pub table: HashMap<String, bool>,
}

//we're actually going to do something kinda weird here.
//we should save each flag as a seperate keyvalue somehow, probably with a debug print.
//we need the Flags datastructure to be heavily modifiable. I don't want the keyvalue table entry to break
//every time someone adds a new bool to it.

pub struct SaveFlags;

pub struct InitFlags;

fn save_flag_process (
    mut pkv: ResMut<PkvStore>,
    mut save_evr: EventReader<SaveFlags>,

    flags: Res<FlagsTable>,
)
{
    for _ in save_evr.iter() {
        info!("saving all flags to memory...");
        for (key, value) in flags.table.iter() {
            info!("saving {} at key {} into memory...", key.clone(), value.clone());

            pkv.set::<bool>(key, value);
        }
    }
}

fn init_flag_process (
    pkv: Res<PkvStore>,
    mut init_evr: EventReader<InitFlags>,

    mut flags: ResMut<FlagsTable>,
)
{
    for _ in init_evr.iter() {
        info!("loading all keys from memory into the flagtable...");
        for (key, _) in flags.table.clone().iter() {
            //we should really only be putting bools in the flag table.
            if let Ok(new_value) = pkv.get::<bool>(&key.clone()) {
                info!("found {} at key {}! inserting...", new_value.clone(), key.clone());

                flags.table.insert(key.clone(), new_value);
            }
        }
    }
}

fn flag_initter (
    mut init_evw: EventWriter<InitFlags>,
)
{
    init_evw.send(
        InitFlags
    );
}
