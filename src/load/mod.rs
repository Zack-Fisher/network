use bevy::{prelude::*, log::Level};

pub struct LoadPlugin;

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(LoadState::Playing)

            .add_event::<LoadLevelEvent>()
            .add_event::<FinishLoad>()

            .add_system(level_loader)
            .add_system(level_load_finisher)
            .add_system(monitor_loadstate)
            ;
    }
}

use crate::physics::*;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum LoadState {
    Playing,
    InitLoad,
    CollisionGen,
    MeshGen,
}

fn monitor_loadstate(
    load_state: Res<State<LoadState>>,
    delta: Res<Time>,
    mut counter: Local<f32>,
)
{
    //slowed print, as to not flood stdout
    *counter += delta.delta_seconds();
    if *counter > 0.2 {
        println!("{:?}", load_state);
        *counter = 0.0;
    }
}

use serde::*;
use serde_json;
use std::fs;

pub fn write_leveldata_to_file(level_data: LevelData, path: String) 
{
    let json_string = serde_json::to_string(&level_data).unwrap();

    match fs::write(&path, json_string){
        Ok(()) => {}
        Err(err) => {error!("{}", err); return;}
    };
}

use std::fs::File;
use std::io::BufReader;

//FILE IS NOT SUPPORTED ON WASM.
//DO NOT USE IT IN PROD
//how can we load arbitrary data from the assetserver?
pub fn get_leveldata_from_file(path: String) -> Option<LevelData>
{
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let level_data = serde_json::from_reader(reader).unwrap();

    level_data
}

#[derive(Resource, Serialize, Deserialize)]
pub struct LevelData {
    pub name: String,
    pub scene_path: String,
}

pub struct LoadLevelEvent {
    //serialize it!!!
    pub level_path: String,
}

impl Default for LoadLevelEvent {
    fn default() -> Self {
        LoadLevelEvent { 
            level_path: String::from("foo.scn.ron"),
        }
    }
}

fn level_loader (
    mut load_state: ResMut<State<LoadState>>,

    mut loadlevel_evr: EventReader<LoadLevelEvent>,

    mut buildlevel_evw: EventWriter<BuildLevelEvent>,
)
{
    for ev in loadlevel_evr.iter() {
        load_state.set(LoadState::InitLoad);

        let level_data = get_leveldata_from_file(ev.level_path.clone()).unwrap();
        info!("now loading level [{}] at path [{}]", level_data.name, ev.level_path.clone());

        info!("initializing a level load...");
        buildlevel_evw.send(BuildLevelEvent { 
            path: level_data.scene_path,
        });
    }
}

pub struct FinishLoad;

fn level_load_finisher (
    mut load_state: ResMut<State<LoadState>>,

    mut finishbuild_evr: EventReader<FinishLoad>,
)
{
    for ev in finishbuild_evr.iter() {
        load_state.set(LoadState::Playing);
    }
}
