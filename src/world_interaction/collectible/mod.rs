use bevy::prelude::*;

use crate::{inventory::ItemData, player_control::player_embodiment::Player};

use super::analysis::AnalyseBundle;

pub struct CollectiblePlugin;

impl Plugin for CollectiblePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(collectible_system)
            ;
    }
}

//many things should be collectible, but they should all work in relatively the same way.
//objects that are run into are collected, and some need to be analyzed. 
//both options should always be valid.

#[derive(Component)]
pub struct Collectible {
    // how far away do we check for collision with the player?
    pub range: f32,
    pub item: ItemData,
    pub is_collected: bool,
}

#[derive(Bundle)]
pub struct CollectibleBundle {
    pub analyse: AnalyseBundle, 
}

fn collectible_system ( 
    mut coll_q: Query<(&GlobalTransform, &mut Collectible)>,
    player_q: Query<&GlobalTransform, With<Player>>,
)
{
    for (gtf, mut coll) in coll_q.iter_mut() {
        if coll.is_collected {continue;}

        for p_gtf in player_q.iter() {
            let dist = (p_gtf.translation() - gtf.translation()).length();

            if dist < coll.range {
                //pull this out into a seperate "collected" event line, this needs to be called by the analyse bundle as well.
                coll.is_collected = true;
            }
        }
    }
}
