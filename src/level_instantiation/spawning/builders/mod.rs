use bevy::{prelude::*, utils::HashMap};
use oxidized_navigation::NavMeshAffector;

use crate::{world_interaction::{analysis::{Analysis, AnalysisEvent, event::EventTypes}, spawnpoint::{Warp, parse_level_from_string, Spawn}}, GameState, util::trait_extension::MeshExt};

use super::objects::{analysis::AnalysisPrefab, GameCollisionGroup};

/// there are, necessarily, a bunch of prefabs that will be loaded in based on the blender glb tagging system.
/// we'll let a "builder" system be the Added<Name> nametag handler systems.
/// these are specifically the ones that init various prefabs not associated with any system in particular.
pub struct BuildersPlugin;

impl Plugin for BuildersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(tag_handler)

            .add_system(build_warp)

            .add_event::<BuildSpawn>()
            .add_event::<BuildWarp>()
            ;
    }
}

fn tag_handler (
    name_q: Query<(Entity, &Name), Added<Name>>,

    mut commands: Commands,

    mut spawn_evw: EventWriter<BuildSpawn>,
    mut warp_evw: EventWriter<BuildWarp>,
)
{
    for (ent, name) in name_q.iter() {
        let mut keyval_table: HashMap<String, String> = HashMap::new();

        let mut current_key = String::new();
        let mut current_value = String::new();
        let mut is_key = false;
        let mut done_with_prefix = false;

        for c in name.chars() {
            if c == '[' {
                is_key = true;
                done_with_prefix = true;
            } else if c == '=' {
                is_key = false;
            } else if c == ']' {
                // Here you can decide what to do with each extracted key-value pair.
                // For this example, I'm just printing them out to the console.
                println!("{} = {}", current_key, current_value);

                keyval_table.insert(current_key.clone(), current_value.clone());

                current_key = String::new();
                current_value = String::new();
                is_key = true;
            } else {
                if done_with_prefix {
                    if is_key {
                        current_key.push(c);
                    } else {
                        current_value.push(c);
                    }
                }
            }
        }

        let mut analysis = Analysis::default();

        for (key, value) in keyval_table.iter() {
            let mut event = AnalysisEvent::default();

            let parsed_value: Vec<&str> = value.split(",").collect();

            let mut is_analysis_tag = false;

            match key.to_lowercase().as_str() {
                //analysis events
                "door" => {
                    event.ev_type = EventTypes::Door;
                    is_analysis_tag = true;
                },
                "levelanim" => {
                    event.ev_type = EventTypes::LevelAnim;
                    is_analysis_tag = true;
                },

                //not analysis events, do something different, probably trigger some far-off eventline
                "warp" => {
                    if parsed_value.len() != 2 {
                        warn!("bad warp. you need two parameters for the warp, you passed {:?}", parsed_value);
                        warn!("example: 'examplewarp [warp=levelname,spawnname]'");
                        continue;
                    }
                    warp_evw.send (
                        BuildWarp {
                            name: parsed_value[0].to_string(),
                            level_name: parsed_value[1].to_string(),
                            warp_ent: ent.clone(),
                        }
                    )
                },
                "spawn" => {
                    spawn_evw.send (
                        BuildSpawn {
                            // we really don't want this to fail no matter what, or it'll mess up our counting.
                            name: parsed_value.join("").to_string(),
                            spawn_ent: ent.clone(),
                        }
                    )
                },
                _ => {continue;}
            }

            if is_analysis_tag {
                // split() will always return an iterator with more than 0 values.
                event.payload = parsed_value[0].to_string();

                analysis.events.push(
                    event
                );
            }
        }

        if analysis.events.len() != 0 {
            info!("done scanning the Name, adding... ({}), ({:?})", name, analysis);

            commands.entity(ent)
                .insert(
                    AnalysisPrefab {analysis}
                );
        }
    }
}

use bevy_rapier3d::prelude::*;

/// event line, when sent, attaches a warpbundle to the specified entity.
#[derive(Debug)]
pub struct BuildWarp {
    pub name: String,
    pub level_name: String,
    pub warp_ent: Entity,
}

fn build_warp (
    mut warp_evr: EventReader<BuildWarp>,

    children: Query<&Children>,
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>>,
    mut commands: Commands,
)
{
    for ev in warp_evr.iter() {
        //find the name, and construct the actual Warp component.
        let mut warp = Warp::default();
        
        warp.name = ev.name.clone();

        match parse_level_from_string(ev.name.clone()) {
            Some(level) => {
                warp.level = level.clone();
            }
            None => {continue;}
        }

        // build the collider for the mesh.
        for (collider_entity, collider_mesh) in
            Mesh::search_in_children(ev.warp_ent, &children, &meshes, &mesh_handles)
        {
            let rapier_collider =
                match Collider::from_bevy_mesh(collider_mesh, &ComputedColliderShape::TriMesh) {
                    Some(collider) => {collider},
                    None => {warn!("tried to build a warp from a mesh with the [warp] tag, but failed to make the mesh. {:?}, {:?}", ev, ev.warp_ent.clone()); continue;}
                };

            commands
                .entity(collider_entity)
                .insert((
                    rapier_collider, 
                    NavMeshAffector::default(),
                    Sensor,
                    ActiveEvents::COLLISION_EVENTS,
                    warp.clone(),
                    Group::from(GameCollisionGroup::WARP),
                )
                );
        }
    }
}
//building spawnpoints is more related to the resource table, so it's in the world_interaction module.

pub struct BuildSpawn {
    pub name: String,
    pub spawn_ent: Entity,
}

