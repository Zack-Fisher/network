use bevy::prelude::*;

pub struct LoadPlugin;

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(LoadState::Playing)
            .add_event::<LoadLevelEvent>()

            .add_system(level_loader);
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

pub struct LoadLevelEvent {
    //serialize it!!!
    scene_path: String,
}

fn level_loader (
    mut load_state: ResMut<State<LoadState>>,

    mut loadlevel_evr: EventReader<LoadLevelEvent>,
)
{
    for ev in loadlevel_evr.iter() {
        load_state.set(LoadState::InitLoad);
        
        load_state.set(LoadState::Playing);
    }
}