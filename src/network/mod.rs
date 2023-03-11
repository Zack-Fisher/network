use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use rand::Rng;

use crate::player_control::{action_handler::ActionStream, actions::PlayerAction};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app
            ;        
    }
}

//the basic idea here is to throw components at ghosts and push into their action vecdeque.
#[derive(Component)]
pub struct RandomMovement {
    count_time: f32,
    cooldown: f32,
}

impl Default for RandomMovement {
    fn default() -> Self {
        Self {
            count_time: 0.0,
            cooldown: 1.0,
        }
    }
}

fn process_random (
    mut random_q: Query<(&mut RandomMovement, &mut ActionStream)>,

    time: Res<Time>,
)
{
    let mut rng = rand::thread_rng();

    let time_passed = time.delta_seconds();
    for (mut random_comp, mut action_stream) in random_q.iter_mut() {
        random_comp.count_time += time_passed;
        random_comp.count_time += rng.gen_range(0.01..=1.0);

        if random_comp.count_time >= random_comp.cooldown {
            action_stream.actions.push_front(ActionState<PlayerAction>);

            random_comp.count_time = 0.0;
        }
    }
}
