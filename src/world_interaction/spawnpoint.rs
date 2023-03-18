use bevy::{prelude::*, utils::HashMap};
use strum::IntoEnumIterator;

use crate::{level_instantiation::{level::Levels, spawning::builders::BuildSpawn}, player_control::player_embodiment::Player, file_system_interaction::level_serialization::{WorldLoadRequest, CurrentLevel}};

pub struct SpawnpointPlugin;

impl Plugin for SpawnpointPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SpawnTable::default())

            .add_system(manage_warp_collisions)
            .add_system_to_stage(CoreStage::PostUpdate, build_spawnpoint)
            ;
    }
}

#[derive(Component, Default, Debug, Clone)]
pub struct Spawn {
    pub name: String,
}

#[derive(Component, Default, Debug, Clone)]
pub struct Warp {
    pub name: String,
    pub level: Levels,
}

/// make the spawn, then add the collision to the mesh.
#[derive(Bundle)]
pub struct SpawnBundle {
    pub spawn: Spawn,
}

pub fn parse_level_from_string (
    name: String,
) -> Option<Levels>
{
    //we'll not worry about the case of overlapping level names, and just choose the first occurrence.
    for level in Levels::iter() {
        if name.to_lowercase() == format!("{:?}", level).to_lowercase() {
            return Some(level.clone());
        }
    }

    warn!("you probably misspelled a level name. you passed the string {}, and we couldn't find anything.", name.clone());
    None
}

use bevy_rapier3d::prelude::*;

fn manage_warp_collisions (
    mut col_evr: EventReader<CollisionEvent>,

    warp_q: Query<&Warp>,
    player_q: Query<Entity, With<Player>>,

    mut load_evw: EventWriter<WorldLoadRequest>,
)
{
    for ev in col_evr.iter() {
        match ev {
            CollisionEvent::Started(ent_one, ent_two, flags) => {
                if let Some(player_e) = player_q.iter().find(|&e| e == ent_one.clone() || e == ent_two.clone()) {
                    let mut my_warp = Option::<Warp>::default();

                    if player_e == ent_one.clone() {
                        if let Ok(warp) = warp_q.get(ent_two.clone()) {my_warp = Some(warp.clone())}
                    } else {
                        if let Ok(warp) = warp_q.get(ent_one.clone()) {my_warp = Some(warp.clone())}
                    }

                    info!("warp collision detected, my_warp {:?}", my_warp.clone());

                    if let Some(warp) = my_warp {
                        load_evw.send(
                            WorldLoadRequest { level: warp.level.clone(), spawnpoint_name: warp.name }
                        )
                    }
                }
            },
            CollisionEvent::Stopped(ent_one, ent_two, flags) => {

            }
        } 
    }
}

/// maybe consider putting the spawntable in the currentlevel resource? currently, we refresh the spawntable in the level_serialization.rs module.
#[derive(Resource, Default)]
pub struct SpawnTable {
    pub table: HashMap<String, Transform>
}

fn build_spawnpoint (
    mut table: ResMut<SpawnTable>,

    tf_q: Query<&GlobalTransform>,

    mut spawn_evr: EventReader<BuildSpawn>,

    mut curr_level: ResMut<CurrentLevel>,
)
{
    for ev in spawn_evr.iter() {
        if let Ok(tf) = tf_q.get(ev.spawn_ent) {
            table.table.insert(ev.name.clone(), Transform::from(*tf));

            info!("inserting spawn into spawntable. table: {:?}", table.table.clone());
            curr_level.curr_spawnpoints += 1;
        }
    }
}
