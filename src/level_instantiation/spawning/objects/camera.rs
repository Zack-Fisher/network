use crate::player_control::actions::*;
use crate::player_control::camera::IngameCamera;
use anyhow::Result;
use bevy::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Default, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct CameraPrefab {

}

//the camera should never be directly related to its own transform.
//the ingame camera has a thirdperson camera type which indirectly dictates the transform
//of the actual camera object.
//this way, our "ghosts" can simulate the movement of an actual camera without the actual viewport.
pub fn build_camera (
    mut commands: Commands,

    prefab_q: Query<(Entity, &CameraPrefab), Added<CameraPrefab>>,
)
{
    for (ent, camera_prefab) in prefab_q.iter() {
        commands.entity(ent)
            .with_children(|children| {
                children
                    .spawn((
                        IngameCamera::default(),
                        Camera3dBundle {
                            ..default()
                        },
                        //they have the same input bundle now.
                        create_player_action_input_manager_bundle(),
                        Name::new("Main Camera"),
                    ))
                    ;
            });
    }
}
