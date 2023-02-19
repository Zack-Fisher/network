use bevy::prelude::*;

pub mod mover;
pub mod npc;
pub mod dialogue;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build (&self, app: &mut App) {
        app
            .add_plugin(dialogue::DialoguePlugin)
            .add_plugin(npc::NPCPlugin)
            .add_plugin(mover::MoverPlugin);
    }
}