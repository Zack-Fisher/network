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

#[derive(Component)]
pub struct TextEntityLink {
    /// this should always be the Entity of the associated textbox with the entity.
    /// elinks are one-directional.
    pub entity: Entity,
}

#[derive(Default, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct NPCPrefab {

}

/// just a marker for more efficient query filters.
#[derive(Component)]
pub struct Textbox;

pub fn build_npc (
    mut commands: Commands,
    animations: Res<AnimationAssets>,
    scenes: Res<SceneAssets>,

    prefab_q: Query<(Entity, &NPCPrefab), Added<NPCPrefab>>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
)
{
    for (ent, npc_prefab) in prefab_q.iter() {
        info!("building the NPC");
        commands.entity(ent)
            .with_children(
                |children| {
                    //now, spawn the textbox and link them seperately.
                    //we need to be careful about tbox elinks just like the ghost cameras, we'll have a ton of tboxes and parent ents.
                    //i don't want the tbox to be a direct child of the NPC.
                    let tbox_ent = children
                        .spawn(
                            PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Plane {size: 1.0})),
                                //modify the StandardMaterial texturehandle to show text.
                                material: mats.add(StandardMaterial {
                                    base_color_texture: None,
                                    ..default()
                                }),
                                ..default()
                            }
                        )
                        .insert(Text2dBundle::default())
                        .insert(Textbox)
                        .id()
                        ;

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
                            TextEntityLink {
                                //now, by having access to the npc entity we automatically get the tbox entity for free.
                                entity: tbox_ent,
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
                                        scale: Vec3::splat(1.0),
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
