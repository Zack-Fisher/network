use crate::level_instantiation::spawning::objects::ghost::GhostPrefab;
use crate::level_instantiation::spawning::objects::npc::NPCPrefab;
use crate::level_instantiation::spawning::objects::player::PlayerPrefab;
use crate::level_instantiation::spawning::objects::race::FlushCheckpointCounts;
use crate::world_interaction::spawnpoint::SpawnTable;
use crate::{file_system_interaction::asset_loading::LevelAssets, level_instantiation::level::Levels};
use crate::util::log_error::log_errors;
use crate::world_interaction::condition::ActiveConditions;
use crate::world_interaction::dialog::CurrentDialog;
use crate::world_interaction::interactions_ui::InteractionOpportunities;
use anyhow::Result;
use bevy::asset::LoadState;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;

use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

pub struct LevelSerializationPlugin;

impl Plugin for LevelSerializationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<WorldLoadRequest>()
            .insert_resource(CurrentLevel::default())
            .add_system_to_stage(CoreStage::PostUpdate, load_world.pipe(log_errors))
            .add_system(poll_loaded) 
            .add_system(load_player)
            ;
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Serialize, Deserialize, Default, Hash)]
#[reflect(Serialize, Deserialize)]
//pass in the filename to a serialized .lvl.ron file.
pub struct WorldLoadRequest {
    pub level: Levels,
    pub spawnpoint_name: String,
}

#[derive(Resource)]
pub struct CurrentLevel {
    pub glb: Handle<Gltf>,
    pub glb_scene: Handle<Scene>,
    pub path: String,
    pub eid: Entity,
    /// the entity that the level's main AnimationPlayer happens to be attached to.
    pub anim_player_eid: Entity,
    /// when we haven't counted, it's initialized to None.
    /// when this is None, don't try to init the player! we haven't grabbed the spawn table properly yet.
    pub spawnpoint_count: Option<u32>,
    /// if curr_spawnpoints is greater than or equal to the spawnpoint count, we can safely spawn the player.
    pub curr_spawnpoints: u32,
    pub spawnpoint_name: String,
    pub is_player_loaded: bool,
}

impl Default for CurrentLevel {
    fn default() -> Self {
        Self {
            glb: Handle::<Gltf>::default(),
            glb_scene: Handle::<Scene>::default(),
            path: String::default(),
            eid: Entity::from_raw(0),
            anim_player_eid: Entity::from_raw(0),
            spawnpoint_count: None,
            curr_spawnpoints: 0,
            // don't bother making this an option. the world load system will always fill this with something.
            // it's nonsense to have a None here, and if it's invalid, just spawn the player at the origin.
            spawnpoint_name: String::from(""),
            is_player_loaded: false,
        }
    }
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

/// the bits that show up no matter what.
/// like the player, the camera, ...
#[derive(Component)]
struct ConstantLevel;

fn load_world (
    mut commands: Commands,
    mut load_requests: EventReader<WorldLoadRequest>,
    mut racetable_flush_evw: EventWriter<FlushCheckpointCounts>,
    levels: Res<Assets<SerializedLevel>>,
    level_handles: Option<Res<LevelAssets>>,
    server: Res<AssetServer>,
    mut curr_level: ResMut<CurrentLevel>,
    mut spawntable: ResMut<SpawnTable>,

    static_q: Query<Entity, With<StaticLevel>>,
    dynamic_q: Query<Entity, With<DynamicLevel>>,
    constant_q: Query<Entity, With<ConstantLevel>>,
) -> Result<()> 
{
    let level_handles = match level_handles {
        Some(level_handles) => level_handles,
        None => {
            return Ok(());
        }
    };

    for load_ev in load_requests.iter() {
        //refresh the currentlevel resource to the default.
        *curr_level = CurrentLevel::default();

        curr_level.spawnpoint_name = load_ev.spawnpoint_name.clone();

        spawntable.table = HashMap::new();

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
        
        //we want to pass around the gltf data for the level on the resource line, other modules need to access
        //things like the animation names of our level.
        let glb_handle: Handle<Gltf> = server.load(format!("{}", serialized_level.scene_path.clone()));

        //load the whole scene
        curr_level.glb = glb_handle.clone();
        curr_level.path = filename.to_string().clone();

        let glb_scene_handle: Handle<Scene> = server.load(format!("{}#Scene0", serialized_level.scene_path.clone()));

        curr_level.glb_scene = glb_scene_handle.clone();

        let static_ent = commands
            .spawn(
                SceneBundle {
                    scene: glb_scene_handle.clone(),
                    ..default()
                }
            )
            .insert(StaticLevel)
            .insert(Name::new("level static scene"))
            .id()
            ;

        curr_level.eid = static_ent.clone();

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

        info!("spawning the player/constant bundle");

        commands
            .spawn(
                SpatialBundle::default()
            )
            .with_children(|children| {
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

/// only load the player in once both the world and all of the spawnpoints have been properly initialized.
fn load_player (
    con_q: Query<Entity, With<ConstantLevel>>,
    mut commands: Commands,

    table: Res<SpawnTable>,

    mut curr_level: ResMut<CurrentLevel>,
)
{
    if curr_level.is_player_loaded {return;}

    if let Some(n) = curr_level.spawnpoint_count {
        info!("{}, {}", n, curr_level.curr_spawnpoints.clone());
        if curr_level.curr_spawnpoints >= n {
            let target = table.table.keys().find(|&s| s.to_lowercase().trim().contains(curr_level.spawnpoint_name.as_str()));
            info!("target acquired: {:?}", target);

            let mut init_transform = Transform::default();

            if let Some(target_string) = target {
                init_transform = table.table.get(target_string).unwrap().clone();
            } else {
                warn!(
                    "target is None, spawning player at the identity gtf: spawnpoint_name {}, table.keys() {:?}",
                    curr_level.spawnpoint_name.clone(), table.table.clone()
                );
            }

            info!("spawning player at transform {:?}", init_transform.clone());

            for ent in con_q.iter() {
                commands.entity(ent)
                    .with_children(|children| {
                        children
                            .spawn(
                                //init_transform on the playerprefab appears to literally do nothing.
                                //lol trolling myself
                                PlayerPrefab::default() 
                            )
                            .insert(
                                SpatialBundle {
                                    transform: init_transform,
                                    ..default()
                                }
                            )
                            ;
                    });
                
                curr_level.is_player_loaded = true;
                //just in case, don't spawn two players under each extra Constant section.
                return;
            }

        }
    }
}

/// update some of the stuff in currlevel accordingly when the level actually finishes loading.
fn poll_loaded (
    mut curr_level: ResMut<CurrentLevel>,

    child_q: Query<&Children>,
    anim_q: Query<&AnimationPlayer>,

    server: Res<AssetServer>,

    gltf: Res<Assets<Gltf>>,
)
{
    use crate::util::*;

    match server.get_load_state(curr_level.glb_scene.clone()) {
        LoadState::Loaded => {
            //try to find the nearest animationplayer of the glb scene.
            match find_type_in_children::<AnimationPlayer> (
                curr_level.eid.clone(),
                &child_q,
                &anim_q,
            ) {
                Some(ent) => {curr_level.anim_player_eid = ent},
                None => {},
            }

            for key in gltf.get(&curr_level.glb).unwrap().named_meshes.keys() {
                // we'll count up all the occurrences of the "spawn" tag.
                let mut temp: u32 = 0;

                if key.clone().trim().to_lowercase().contains("[spawn=") {
                    info!("found spawn tag, adding to temp");
                    temp += 1;
                }

                curr_level.spawnpoint_count = Some(temp.clone());
            }
        },
        _ => {}
    }
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
