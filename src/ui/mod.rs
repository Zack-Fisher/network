use bevy::prelude::*;

pub mod raceui;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(raceui::RaceUIPlugin)
            ; 
    }
}