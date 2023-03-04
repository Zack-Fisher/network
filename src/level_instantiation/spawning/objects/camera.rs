use crate::player_control::actions::create_camera_action_input_manager_bundle;
use crate::player_control::camera::IngameCamera;
use anyhow::Result;
use bevy::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Default, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct CameraPrefab {

}

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
                        create_camera_action_input_manager_bundle(),
                        Name::new("Main Camera"),
                    ))
                    ;
            });
    }
}
