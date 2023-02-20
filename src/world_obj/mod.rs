//for the processing of miscellaneous prefabs, pretty much.
use bevy::prelude::*;

pub mod racetimer;

pub struct WorldObjectPlugin;

impl Plugin for WorldObjectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(racetimer::TimerPlugin);
    }
}