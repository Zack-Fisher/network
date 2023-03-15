//detects all the wacky things that happen to npcs, and signals them out.
//one part of the dialogue system.
use bevy::prelude::*;
use bevy_text_mesh::TextMesh;

use crate::{level_instantiation::spawning::objects::npc::NPCData, player_control::player_embodiment::Player};

use super::textbox::ShowText;

pub struct NPCDetectionPlugin;

impl Plugin for NPCDetectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(is_in_range)

            .add_system(npc_textshow_process)
            ;
    }
}

fn npc_textshow_process (
    data_q: Query<(&NPCData, &Children)>,

    mut textmesh_q: Query<&mut TextMesh>,
)
{
    for (data, children) in data_q.iter() {
        for &child in children {
            if let Ok(mut npc_textmesh) = textmesh_q.get_mut(child.clone()) {

            }
        } 
    }
}

//data is attached to objects, not functionality.
//we can't really think of an npc "handling" detection, but we can have it hold all the necessary
//detection data.

/// is it worth having the data and emitter systems be seperate?
/// these are systems that operate on each npc and their NPCData component.
fn is_in_range (
    mut npc_q: Query<(&mut NPCData, &GlobalTransform, Entity)>,
    player_q: Query<&GlobalTransform, With<Player>>,

    mut textsend_evw: EventWriter<ShowText>,
)
{
    for (mut npc_data, npc_gtf, npc_ent) in npc_q.iter_mut() {
        for player_gtf in player_q.iter() {
            let distance = (player_gtf.translation() - npc_gtf.translation()).length();

            if distance <= 3.0 {
                if !npc_data.is_in_range {
                    //then, this is a fresh activation
                    textsend_evw.send(
                        ShowText {
                            char_entity: npc_ent,
                            text: String::from("you're in my range, newly speaking."),
                        }
                    );
                }
                npc_data.is_in_range = true;
            } else {
                npc_data.is_in_range = false;
            }
        }
    }
}
