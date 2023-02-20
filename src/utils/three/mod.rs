use bevy::prelude::*;

use std::default::Default;

use crate::player::player_controller::*;

pub struct ThreePlugin;

//enables all the base functionality of the "three" util class.
impl Plugin for ThreePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(billboard_update)
            .add_system(handle_rotation);
    }
}

#[derive(Component)]
pub struct Rotation {
    pub speed_x: f32,
    pub speed_y: f32,
    pub speed_z: f32,
}

impl Default for Rotation {
    fn default() -> Rotation {
        Rotation {
            speed_x: 0.01,
            speed_y: 0.07,
            speed_z: 0.01,
        }
    }
}

impl Rotation {
    pub const Y: Rotation = Rotation {
        speed_x: 0.0,
        speed_y: 0.05,
        speed_z: 0.0,
    };
}

fn handle_rotation(
    mut transforms: Query<(&mut Transform, &Rotation)>,
) {
    for (mut transform, rotation) in transforms.iter_mut() {
        transform.rotate_x(rotation.speed_x);
        transform.rotate_y(rotation.speed_y);
        transform.rotate_z(rotation.speed_z);
    }
}

//billboards an object toward the player.
#[derive(Component)]
pub enum Billboard {
    Y_BILL,
    FULL_BILL,
}

fn billboard_update(
    mut bill_query: Query<(&Billboard, &mut Transform, &GlobalTransform), Without<PlayerCamera>>,
    player_tf_q: Query<&GlobalTransform, With<PlayerCamera>>,
)
{
    let cam_gtf = player_tf_q.single();
    for (board, mut bill_tf, bill_gtf) in bill_query.iter_mut()
    {
        let goal = Transform::from_translation(bill_tf.translation)
            .look_at(cam_gtf.translation(), Vec3::Y);

        // bill_tf = goal;
    }
}