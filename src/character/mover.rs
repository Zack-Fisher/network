use bevy::prelude::*;

use crate::network::world_controller::MoverMessage;

use rand::prelude::*;

use rand::seq::IteratorRandom;

pub struct MoverPlugin;

impl Plugin for MoverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(compute_mover);
    }
}

#[derive(Component)]
pub struct Mover {
    pub speed: f32,
}

fn compute_mover(
    mut mover_tf: Query<(&mut Transform, &Mover)>,
    mut mover_messages: EventReader<MoverMessage>,
) {
    let mut rng = thread_rng();

    //it only calculates the direction for the first mover on an event, then it returns.
    
    let (mut transform, mover) = mover_tf.iter_mut().choose(&mut rng).unwrap();

    for mover_message in mover_messages.iter() {
        let computed = mover_message.direction;
        let mut direction = Vec3::from_array([computed[0], 0.0, computed[1]]);
        direction = direction.normalize() * mover.speed;
        transform.translation += direction;
        return;
    }
}