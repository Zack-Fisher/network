use bevy::prelude::*;

pub mod mover;
pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build (&self, app: &mut App) {
        app
            .add_plugin(mover::MoverPlugin);
    }
}