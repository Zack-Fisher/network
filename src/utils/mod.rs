pub mod three;
pub mod two;
pub mod ui;

use bevy::prelude::*;

pub struct DefaultUtilPlugin;

//enables all util functionality.
//can be ported between projects, ideally.
impl Plugin for DefaultUtilPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(three::ThreePlugin);
    }
}