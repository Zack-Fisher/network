use crate::file_system_interaction::asset_loading::*;
use crate::movement::general_movement::{CharacterAnimations, CharacterControllerBundle, Model};
use crate::player_control::actions::{
    create_player_action_input_manager_bundle, create_ui_action_input_manager_bundle,
};
use crate::player_control::player_embodiment::Player;
use anyhow::Result;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use std::f32::consts::TAU;

use serde::{Serialize, Deserialize};

use super::GameCollisionGroup;

pub const HEIGHT: f32 = 0.4;
pub const RADIUS: f32 = 0.3;

//to be added through the editor and spawned in a serialized dynamic scene.
#[derive(Default, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct PlayerPrefab {

}

//if necessary, we can also put a player_process method that pulls from the PlayerPrefab member variables. 
//this should be good enough for roughly any use case?

//whenever it detects a new PlayerPrefab added in the scene, the Player function is built as a child.
//should be pretty flexible.
pub fn build_player(
    mut commands: Commands,
    animations: Res<AnimationAssets>,
    scenes: Res<SceneAssets>,

    prefab_q: Query<(Entity, &PlayerPrefab), Added<PlayerPrefab>>,
)
{
    for (ent, player_prefab) in prefab_q.iter() {
        commands.entity(ent).with_children(|children| {
            children
                .spawn((
                    PbrBundle {
                        ..default()
                    },
                    Player,
                    Name::new("Player"),
                    CharacterControllerBundle::capsule(HEIGHT, RADIUS),
                    CharacterAnimations {
                        idle: animations.character_idle.clone(),
                        walk: animations.character_walking.clone(),
                        aerial: animations.character_running.clone(),
                    },
                    CollisionGroups::new(
                        GameCollisionGroup::PLAYER.into(), GameCollisionGroup::ALL.into(),
                    ),
                    Ccd::enabled(),
                    create_player_action_input_manager_bundle(),
                    create_ui_action_input_manager_bundle(),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        SceneBundle {
                            scene: scenes.character.clone(),
                            transform: Transform {
                                translation: Vec3::new(0., -HEIGHT / 2. - RADIUS, 0.),
                                rotation: Quat::from_rotation_y(TAU / 2.),
                                scale: Vec3::splat(0.01),
                            },
                            ..default()
                        },
                        Model,
                        Name::new("Player Model"),
                    ));
                });
            });
    }
}
