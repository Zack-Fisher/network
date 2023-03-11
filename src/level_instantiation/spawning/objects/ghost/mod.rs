use crate::GameState;
use crate::file_system_interaction::asset_loading::*;
use crate::level_instantiation::spawning::animation_link::AnimationEntityLink;
use crate::movement::general_movement::{CharacterAnimations, CharacterControllerBundle, Model, CameraEntityLink};
use crate::player_control::action_handler::ActionStream;
use crate::player_control::actions::{
    create_player_action_input_manager_bundle, create_ui_action_input_manager_bundle,
};
use crate::player_control::camera::IngameCamera;
use crate::player_control::player_embodiment::Player;
use crate::ui::mapui::MapHandle;
use anyhow::Result;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy_rapier3d::prelude::*;

use std::f32::consts::TAU;

use serde::{Serialize, Deserialize};

use self::components::GhostCamera;

use super::GameCollisionGroup;

pub const HEIGHT: f32 = 0.4;
pub const RADIUS: f32 = 0.3;

pub mod processing;
pub mod components;

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(build_ghost)
                    .with_system(build_ghost_splat)
                    .with_system(processing::ghost_process)
            )
            ;
    }
}

#[derive(Default, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
//several ghosts will spawn with the specified count at this place, just adds the ghostprefab
//around the transform.
pub struct GhostSplat {
    pub max_distance: f32,
    pub count: u32,
}

//to be added through the editor and spawned in a serialized dynamic scene.
#[derive(Default, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
//the init_transform is decided by the world loader in level_serialization.rs
//it's based on the selected warppoint
pub struct GhostPrefab {
}

//if necessary, we can also put a player_process method that pulls from the PlayerPrefab member variables. 
//this should be good enough for roughly any use case?

//whenever it detects a new PlayerPrefab added in the scene, the Player function is built as a child.
//should be pretty flexible.
pub fn build_ghost(
    mut commands: Commands,
    animations: Res<AnimationAssets>,
    scenes: Res<SceneAssets>,

    prefab_q: Query<(Entity, &GhostPrefab), Added<GhostPrefab>>,
)
{
    for (ent, ghost_prefab) in prefab_q.iter() {
        commands.entity(ent).with_children(|children| {
                let camera_entity = children
                    .spawn((
                        SpatialBundle::default(),
                        IngameCamera::default(),
                        Name::new("Ghost Camera"),
                        GhostCamera,
                    ))
                    .id()
                    ;

                let e_com = children
                    .spawn((
                        PbrBundle {
                            ..default()
                        },
                        Name::new("Ghost"),
                        CharacterControllerBundle::capsule(HEIGHT, RADIUS),
                        ActionStream::new(),
                        CharacterAnimations {
                            idle: animations.character_idle.clone(),
                            walk: animations.character_walking.clone(),
                            aerial: animations.character_running.clone(),
                        },
                        CollisionGroups::new(
                            GameCollisionGroup::PLAYER.into(), GameCollisionGroup::ALL.into(),
                        ),
                        Ccd::enabled(),
                        //we're linking the entity to its camera, so that we don't trigger every IngameCamera with each action.
                        CameraEntityLink {camera_entity},
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            SceneBundle {
                                scene: scenes.character.clone(),
                                transform: Transform {
                                    translation: Vec3::new(0., -HEIGHT / 2. - RADIUS, 0.),
                                    rotation: Quat::from_rotation_y(TAU / 2.),
                                    scale: Vec3::splat(1.),
                                },
                                ..default()
                            },
                            Model,
                            Name::new("Ghost Model"),
                        ))
                        ;
                    })
                    ;
            });

    }
}

use rand::Rng;

pub fn build_ghost_splat (
    mut commands: Commands,

    mut prefab_q: Query<(Entity, &GhostSplat), Added<GhostSplat>>,
)
{
    for (ghost_splat, splat_comp) in prefab_q.iter_mut() {
        let mut ent_coms = commands.entity(ghost_splat);

        let mut vec: Vec<Transform> = vec![];

        let mut rng = rand::thread_rng();

        for _ in 1..splat_comp.count {
            vec.push(
                Transform::from_xyz(rng.gen_range(-splat_comp.max_distance..=splat_comp.max_distance), 0.0, rng.gen_range(-splat_comp.max_distance..=splat_comp.max_distance))
            )
        }

        ent_coms.with_children(
            |children| {
                for tf in vec {
                    children
                        .spawn(
                            (
                                SpatialBundle {
                                    transform: tf,
                                    ..default()
                                },
                                GhostPrefab::default(),
                            )
                        );
                }
            }
        );
    }
}
