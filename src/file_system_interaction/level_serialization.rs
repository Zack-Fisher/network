use crate::level_instantiation::spawning::objects::camera::CameraPrefab;
use crate::level_instantiation::spawning::objects::ghost::{GhostPrefab, GhostSplat};
use crate::level_instantiation::spawning::objects::npc::NPCPrefab;
use crate::level_instantiation::spawning::objects::player::PlayerPrefab;
use crate::level_instantiation::spawning::objects::race::FlushCheckpointCounts;
use crate::level_instantiation::spawning::objects::skybox::SkyboxPrefab;
use crate::{file_system_interaction::asset_loading::LevelAssets, level_instantiation::level::Levels};
use crate::util::log_error::log_errors;
use crate::world_interaction::condition::ActiveConditions;
use crate::world_interaction::dialog::CurrentDialog;
use crate::world_interaction::interactions_ui::InteractionOpportunities;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;

use bevy_editor_pls::prelude::InScene;
use serde::{Deserialize, Serialize};

pub struct LevelSerializationPlugin;

impl Plugin for LevelSerializationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<WorldLoadRequest>()
            .add_system_to_stage(CoreStage::PostUpdate, load_world.pipe(log_errors));
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Serialize, Deserialize, Default, Hash)]
#[reflect(Serialize, Deserialize)]
//pass in the filename to a serialized .lvl.ron file.
pub struct WorldLoadRequest {
    pub level: Levels,
    pub spawnpoint_name: String,
}

#[derive(Debug, Clone, PartialEq, Resource, Reflect, Serialize, Deserialize, Default)]
#[reflect(Resource, Serialize, Deserialize)]
pub struct CurrentLevel {
    pub scene: String,
}

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Protected;


////!!we can consider a level to consist of three major parts!!////

//the glb scene that we'll import from blender.
#[derive(Component)]
struct StaticLevel;

//all the entities that move around in the level.
#[derive(Component)]
struct DynamicLevel;

//the bits that show up no matter what.
//like the player, the camera, ...
#[derive(Component)]
struct ConstantLevel;

fn load_world(
    mut commands: Commands,
    mut load_requests: EventReader<WorldLoadRequest>,
    mut racetable_flush_evw: EventWriter<FlushCheckpointCounts>,
    levels: Res<Assets<SerializedLevel>>,
    level_handles: Option<Res<LevelAssets>>,
    server: Res<AssetServer>,

    static_q: Query<Entity, With<StaticLevel>>,
    dynamic_q: Query<Entity, With<DynamicLevel>>,
    constant_q: Query<Entity, With<ConstantLevel>>,
) -> Result<()> {
    let level_handles = match level_handles {
        Some(level_handles) => level_handles,
        None => {
            return Ok(());
        }
    };

    for load_ev in load_requests.iter() {
        //despawn all the world stuff when we make a new request.
        for s_ent in static_q.iter() {
            commands.entity(s_ent).despawn_recursive();
        }

        for d_ent in dynamic_q.iter() {
            commands.entity(d_ent).despawn_recursive();
        }

        for c_ent in constant_q.iter() {
            commands.entity(c_ent).despawn_recursive();
        }

        let filename = load_ev.level.to_string();

        let serialized_level = levels
            .get(level_handles.levels.get(filename).ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Level not found: {}", filename),
                )
            })?)
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Serialized level not found",
                )
            })?;

        commands
            .spawn(
                SceneBundle {
                    scene: server.load(format!("{}#Scene0", serialized_level.scene_path.clone())),
                    ..default()
                }
            )
            .insert(StaticLevel)
            .insert(Name::new("level static scene"))
            ;

        commands
            .spawn(
                DynamicSceneBundle {
                    scene: server.load(serialized_level.dynamic_path.clone()),
                    ..default()
                }
            )
            .insert(DynamicLevel)
            .insert(Name::new("level dynamic scene"))
            ;

        commands
            .spawn(
                SpatialBundle::default()
            )
            .with_children(|children| {
                children
                    .spawn(
                        (
                            PlayerPrefab {
                                //temp
                                //have to pull the PlayerPrefab insertion into another system, because
                                //we need to wait until the dynamicscene (containing the spawnpoints) is loaded.
                                init_transform: Transform::IDENTITY,
                            },
                            SpatialBundle::default(),
                        )
                    );

                children
                    .spawn(
                        SpatialBundle {
                            transform: Transform::from_xyz(0.0, 5.0, 5.0),
                            ..default()
                        }
                    )
                    .insert(
                        GhostPrefab::default()
                    )
                    ;

                children
                    .spawn(
                        SpatialBundle {
                            transform: Transform::from_xyz(-3.0, 1.0, -4.0),
                            ..default()
                        }
                    )
                    .insert(
                        NPCPrefab::default()
                    )
                    ;


            })
            .insert(ConstantLevel)
            .insert(Name::new("Constants"))
            ;

        commands.insert_resource(CurrentLevel {
            scene: filename.to_string(),
        });
        commands.insert_resource(InteractionOpportunities::default());
        commands.insert_resource(ActiveConditions::default());
        commands.remove_resource::<CurrentDialog>();


        //do some extra stuff
        //clean out the counts in the racetable
        racetable_flush_evw.send(FlushCheckpointCounts);

        info!("Successfully loaded scene \"{}\"", filename,)
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, TypeUuid)]
#[uuid = "eb7cc7bc-5a97-41ed-b0c3-0d4e2137b73b"]
#[reflect(Serialize, Deserialize)]
//levels consist of a static Blender-created scene and Entities.
//we seperate it into two parts, as follows, and serialize the levels to .ron
pub struct SerializedLevel {
    pub scene_path: String,
    pub dynamic_path: String,
}
