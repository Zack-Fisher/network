use std::collections::VecDeque;

use bevy::{prelude::*, utils::HashMap};
use leafwing_input_manager::prelude::ActionState;

use crate::recording::RecordingTable;

use super::actions::PlayerAction;

pub struct ActionBusPlugin;

impl Plugin for ActionBusPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ActionMessage>()

            .add_system(apply_action)
            .add_system(choose_action_from_table)
            ;
    }
}

#[derive(Component)]
pub struct ActionStream {
    pub actions: VecDeque<ActionState<PlayerAction>>,
}

impl ActionStream {
    pub fn new() -> Self {
        Self {
            actions: VecDeque::new(),
        }
    }
}

//we send a playeraction to the line and .get() the related components from the query.
//this way, we can apply input transformations to the right entities.
pub struct ActionMessage {
    pub entity: Entity,
    pub action: ActionState<PlayerAction>,
}

//we have an actionbus component stuck to each player object that will read
//line by line each frame and apply those actions in the system.
//the action application process will be locked to the physics framerate by default.
#[derive(Component)]
pub struct ActionBus;

fn apply_action (
    mut action_q: Query<(Entity, &mut ActionStream)>,

    mut action_evw: EventWriter<ActionMessage>,
)
{
    for (entity, mut action_stream) in action_q.iter_mut() {
        match action_stream.actions.pop_front() {
            Some(action) => {
                action_evw.send(
                    ActionMessage {
                        entity,
                        action,
                    }
                )
            }
            None => {

            }
        }
    }
}

use rand::seq::SliceRandom;

/// this is the ActionStream's natural repopulation mechanism.
fn choose_action_from_table (
    mut local_recordingtable: ResMut<RecordingTable>,

    mut action_q: Query<&mut ActionStream>,
)
{
    for mut action_c in action_q.iter_mut() {
        if action_c.actions.len() <= 0 {
            //there's no more actions left! do something about this!

            let keys: Vec<_> = local_recordingtable.table.keys().collect();

            //keys.choose() will return None if the vector has 0 elements.
            if let Some(random_key) = keys.choose(&mut rand::thread_rng()) {
                if let Some(actions_vec) = local_recordingtable.table.get(random_key.clone()) {
                    action_c.actions = actions_vec.clone();
                }
            }
        }
    } 
}
