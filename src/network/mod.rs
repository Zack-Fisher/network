use bevy::prelude::*;

pub mod chatbot;
pub mod world_controller;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(world_controller::WorldControllerPlugin);
    }
}