use bevy_rapier3d::prelude::*;

use bitflags::bitflags;
use bevy::prelude::*;

use crate::GameState;

pub mod player;
pub mod camera;
pub mod npc;
pub mod primitives;
pub mod spawnpoint;
pub mod warppoint;
pub mod skybox;
pub mod race;
pub mod ghost;
pub mod analysis;
pub mod collectible;

pub struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(race::RacePlugin)
            .add_plugin(ghost::GhostPlugin)

            //this plugin runs all the builder systems for each prefab in the game
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(player::build_player)
                    .with_system(player::init_animation_entity_link)
                    .with_system(camera::build_camera)
                    .with_system(npc::build_npc)
                    .with_system(primitives::build_cube)
                    .with_system(skybox::build_skybox)
                    .with_system(skybox::skybox_process)
                    .with_system(analysis::build_analysis)

                    .with_system(collectible::build_collectible)

                    .with_system(warppoint::build_warppoint)
                    .with_system(warppoint::warppoint_process)
            )
            ;
        }
}

//all the collision groups for the game, organized here.
//CollisionGroups has two fields
// commands.spawn(Collider::ball(0.5))
//     .insert(CollisionGroups::new(0b1101.into(), 0b0100.into())
//     .insert(SolverGroups::new(0b0011.into(), 0b1011.into());
//with a maximum of 32 groups, stored in a u32.
bitflags! {
    pub struct GameCollisionGroup: u32 {
        const PLAYER = 1 << 0;
        const OTHER = 1 << 31;
        const WARP = 1 << 2;

        const ALL = u32::MAX;
        const NONE = 0;
    }
}

impl From<GameCollisionGroup> for Group {
    fn from(value: GameCollisionGroup) -> Self {
        // Both are u32, so this will never panic.
        Self::from_bits(value.bits()).expect("Failed to convert GameCollisionGroup to rapier Group")
    }
}