use bevy::prelude::*;
use bevy::utils::Uuid;

use crate::{inventory::ItemData, player_control::player_embodiment::Player, accessories::enums::HatAcc};

use super::analysis::AnalyseBundle;

pub struct CollectiblePlugin;

impl Plugin for CollectiblePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<IsCollected>()

            .insert_resource(CollectibleTable::default())

            .add_system(check_table)
            .add_system(collectible_system)
            .add_system(is_collected_process)
            ;
    }
}

///todo later: serialize the table with pkv, make everything persist.
#[derive(Resource, Default)]
pub struct CollectibleTable {
    pub collected: Vec<String>,
}
//many things should be collectible, but they should all work in relatively the same way.
//objects that are run into are collected, and some need to be analyzed. 
//both options should always be valid.

#[derive(Component)]
pub struct Collectible {
    // how far away do we check for collision with the player?
    pub range: f32,
    pub item: ItemData,
    /// whenever is_collected is Some, we've ensured that it's checked the Uuid table.
    /// when it's None, there's no guarantee either way.
    pub is_collected: Option<bool>,
    pub uuid: String,
}

impl Default for Collectible {
    fn default() -> Self {
        Self {
            range: 3.0,
            item: ItemData::Hat(HatAcc::TopHat),
            is_collected: None,
            uuid: Uuid::new_v4().to_string(),
        }
    }
}

#[derive(Bundle, Default)]
pub struct CollectibleBundle {
    pub analyse: AnalyseBundle, 
    pub collectible: Collectible,
}

/// based on the uuid of the Collectible, make sure that it hasn't already been collected.
fn check_table (
    mut coll_q: Query<(Entity, &mut Collectible), Added<Collectible>>,

    coll_t: Res<CollectibleTable>,
    mut commands: Commands,
)
{
    for (coll_ent, mut coll) in coll_q.iter_mut() {
        if coll_t.collected.contains(&coll.uuid) {
            commands.entity(coll_ent).despawn_recursive();
        } else {
            coll.is_collected = Some(false);
        }
    }
}

fn collectible_system ( 
    mut coll_q: Query<(Entity, &GlobalTransform, &mut Collectible)>,
    player_q: Query<&GlobalTransform, With<Player>>,

    mut coll_evw: EventWriter<IsCollected>,
)
{
    for (c_ent, gtf, mut coll) in coll_q.iter_mut() {
        if let Some(collected) = coll.is_collected {
            if collected {continue;}

            for p_gtf in player_q.iter() {
                let dist = (p_gtf.translation() - gtf.translation()).length();
            
                if dist < coll.range {
                    //pull this out into a seperate "collected" event line, this needs to be called by the analyse bundle as well.
                    coll_evw.send(
                        IsCollected { collectible_ent: c_ent.clone() }
                    )
                }
            }
        }
    }
}

pub struct IsCollected {
    pub collectible_ent: Entity,
}

fn is_collected_process (
    mut col_evr: EventReader<IsCollected>,
    mut col_q: Query<&mut Collectible>,

    mut commands: Commands,

    mut coll_t: ResMut<CollectibleTable>,
)
{
    for ev in col_evr.iter() {
        //just despawn it for now bleh
        if let Ok(mut col) = col_q.get_mut(ev.collectible_ent.clone()) {
            // we've collected it now, push it to the global table so it doesn't show up anymore.
            coll_t.collected.push(col.uuid.clone());

            commands.entity(ev.collectible_ent.clone()).despawn_recursive(); 
        }
    }
}