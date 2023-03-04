use crate::file_system_interaction::asset_loading::LevelAssets;
use crate::level_instantiation::spawning::{
    SpawnTracker,
};
use crate::util::log_error::log_errors;
use crate::world_interaction::condition::ActiveConditions;
use crate::world_interaction::dialog::CurrentDialog;
use crate::world_interaction::interactions_ui::InteractionOpportunities;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;

use bevy_editor_pls::prelude::NotInScene;
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
    pub filename: String,
}

#[derive(Debug, Clone, PartialEq, Resource, Reflect, Serialize, Deserialize, Default)]
#[reflect(Resource, Serialize, Deserialize)]
pub struct CurrentLevel {
    pub scene: String,
}

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Protected;

fn load_world(
    mut commands: Commands,
    mut load_requests: EventReader<WorldLoadRequest>,
    current_spawn_query: Query<Entity, With<SpawnTracker>>,
    levels: Res<Assets<SerializedLevel>>,
    level_handles: Option<Res<LevelAssets>>,
    server: Res<AssetServer>,
) -> Result<()> {
    let level_handles = match level_handles {
        Some(level_handles) => level_handles,
        None => {
            return Ok(());
        }
    };
    for load_ev in load_requests.iter() {
        let serialized_level = levels
            .get(level_handles.levels.get(&load_ev.filename).ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Level not found: {}", load_ev.filename),
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
                DynamicSceneBundle {
                    scene: server.load(serialized_level.dynamic_path.clone()),
                    ..default()
                }
            )
            .insert(Name::new("level dynamic scene"))
            ;
        
        commands
            .spawn(
                SceneBundle {
                    scene: server.load(format!("{}#Scene0", serialized_level.scene_path.clone())),
                    ..default()
                }
            )
            .insert(NotInScene)
            .insert(Name::new("level static scene"))
            ;

        commands
            .spawn(
                Camera3dBundle::default()
            );

        for entity in &current_spawn_query {
            commands
                .get_entity(entity)
                .context("Failed to get entity while loading")?
                .despawn_recursive();
        }

        commands.insert_resource(CurrentLevel {
            scene: load_ev.filename.clone(),
        });
        commands.insert_resource(InteractionOpportunities::default());
        commands.insert_resource(ActiveConditions::default());
        commands.remove_resource::<CurrentDialog>();

        info!("Successfully loaded scene \"{}\"", load_ev.filename,)
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
