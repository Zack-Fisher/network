use bevy::prelude::*;

pub mod main;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(main::MainUIPlugin)        
            ;
    }
}