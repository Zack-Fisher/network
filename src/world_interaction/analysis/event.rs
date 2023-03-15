use bevy::prelude::*;

pub struct AnalysisEventPlugin;

impl Plugin for AnalysisEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(analysis_event_process)
            ;
    }
}

use serde::{Deserialize, Serialize};

use crate::world_interaction::level_anim::LevelAnim;

use super::AnalysisEvent;

/// when clicked, an analysable object will send the payload along the associated event.
/// sort eventtypes by destination, not source!
#[derive(Default, Component, Reflect, Serialize, Deserialize, Debug, Clone)]
#[reflect(Component, Serialize, Deserialize)]
pub enum EventTypes {
    #[default]
    Door,
    LevelAnim,
}

/// use this system to handle the complexity of parsing the analysis events in one place.
/// basically just an event type coercion, doing the automatic translations instead of spreading it out and 
/// letting them all handle it on their own.
fn analysis_event_process (
    mut an_evr: EventReader<AnalysisEvent>,

    mut anim_evw: EventWriter<LevelAnim>,
)
{
    for ev in an_evr.iter() {
        match ev.ev_type.clone() {
            EventTypes::Door => {

            },

            EventTypes::LevelAnim => {
                anim_evw.send(LevelAnim {
                    name: ev.payload.clone(),
                })
            }
        }
    }
}
