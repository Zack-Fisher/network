pub mod three;
pub mod two;
pub mod ui;

pub mod print;

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