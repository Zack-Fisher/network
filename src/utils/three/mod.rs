use bevy::prelude::*;

use std::default::Default;

pub struct ThreePlugin;

//enables all the base functionality of the "three" util class.
impl Plugin for ThreePlugin {
    fn build(&self, app: &mut App) {
        app
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