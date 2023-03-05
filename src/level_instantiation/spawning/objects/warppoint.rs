use bevy::prelude::*;

use bevy_rapier3d::prelude::Collider;
use serde::{Serialize, Deserialize};

use crate::level_instantiation::level::Levels;

#[derive(Default, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct WarppointPrefab {
    //the target spawnpoint name it will search for. instead of making the default Vec3::ZERO,
    //make it automatically choose the first warppoint if not found.
    pub target: String,
    pub level_path: Levels,
}

//bevy editor recognizes enum values, and they're editable live.
//we can use enums for levels instead.

//the player physically collides with the warppoint to go to the specified level.
pub fn build_warppoint (
    mut commands: Commands,

    prefab_q: Query<(Entity, &WarppointPrefab), Added<WarppointPrefab>>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
)
{
    for (ent, warppoint_prefab) in prefab_q.iter() {
        commands.entity(ent)
            .insert(
                Collider::cuboid(1., 1., 1.)
            );
    }
}

pub fn warppoint_process (
    prefab_q: Query<(Entity, &WarppointPrefab), With<WarppointPrefab>>,
)
{
    for (warp_ent, warppoint_prefab) in prefab_q.iter() {
        //todo:: check for collision, emit the load level event.
    }
}
