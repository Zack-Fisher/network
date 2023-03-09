use std::collections::VecDeque;

use bevy::{prelude::*, utils::HashMap};
use leafwing_input_manager::prelude::ActionState;

use super::actions::PlayerAction;

pub struct ActionBusPlugin;

impl Plugin for ActionBusPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ActionMessage>()

            .add_system(apply_action)
            ;
    }
}

//should the player have the same ActionBus mechanism as the ghosts? yes.
//we need the NN to be able to modify the actionstream freely.
//this needs to be a highly flexible process.
//how do we signal the end of an actionstream?
//we need an "empty" action stream.
//i don't think it's that important to have invertible actionstreams.
//it'll be saved to process memory loaded in from computer memory, and we pull the actionlist
//into an actionstream.
//actionstreams are fundementally more flexible than actionlists.
//stack of actions in the actionstream, we "pop" off the actionstack.
//the file format needs init values for the ghosts, but the actionstream doesn't need that.
//just actions, executed from a stack of variants.
//maybe use the get() method on the query by e_id? how can we actually pass the actionstream
//pops to the right characters?

//how can we add the player input to the player's vecdeque without dropping any frames?
//wait it's a collection of multiple actions per frame, not just one.
//need to iterate through actioncollections.
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
