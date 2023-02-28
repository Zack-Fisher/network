//management for the UI elements on the screen during gameplay.
use bevy::prelude::*;

pub mod main;
pub mod map;
pub mod load_screen;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(main::MainUIPlugin)        
            .add_plugin(map::MapPlugin)
            .add_plugin(load_screen::LoadScreen)
            ;
    }
}