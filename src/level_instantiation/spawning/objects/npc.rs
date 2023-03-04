use crate::level_instantiation::spawning::objects::GameCollisionGroup;
use crate::file_system_interaction::asset_loading::*;
use crate::movement::general_movement::{CharacterAnimations, CharacterControllerBundle, Model};
use crate::movement::navigation::Follower;
use crate::world_interaction::dialog::{DialogId, DialogTarget};
use anyhow::Result;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

use serde::{Serialize, Deserialize};

pub const HEIGHT: f32 = 0.4;
pub const RADIUS: f32 = 0.4;

#[derive(Default, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct NPCPrefab {

}

pub fn build_npc (
    mut commands: Commands,
    animations: Res<AnimationAssets>,
    scenes: Res<SceneAssets>,

    prefab_q: Query<(Entity, &NPCPrefab), Added<NPCPrefab>>,
)
{
    for (ent, npc_prefab) in prefab_q.iter() {
        commands.entity(ent)
            .with_children(
                |children| {
                    children
                        .spawn((
                            PbrBundle {
                                ..default()
                            },
                            Name::new("NPC"),
                            CharacterControllerBundle::capsule(HEIGHT, RADIUS),
                            Follower,
                            CharacterAnimations {
                                idle: animations.character_idle.clone(),
                                walk: animations.character_walking.clone(),
                                aerial: animations.character_running.clone(),
                            },
                            DialogTarget {
                                dialog_id: DialogId::new("follower"),
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Name::new("NPC Dialog Collider"),
                                Collider::cylinder(HEIGHT / 2., RADIUS * 5.),
                                Sensor,
                                ActiveEvents::COLLISION_EVENTS,
                                ActiveCollisionTypes::DYNAMIC_DYNAMIC,
                                CollisionGroups::new(
                                    GameCollisionGroup::OTHER.into(),
                                    GameCollisionGroup::PLAYER.into(),
                                ),
                            ));
                            parent.spawn((
                                SceneBundle {
                                    scene: scenes.character.clone(),
                                    transform: Transform {
                                        translation: Vec3::new(0., -HEIGHT / 2. - RADIUS, 0.),
                                        scale: Vec3::splat(0.012),
                                        rotation: Quat::from_rotation_y(TAU / 2.),
                                    },
                                    ..default()
                                },
                                Model,
                                Name::new("NPC Model"),
                            ));
                        });
                }
            );
    }
}
