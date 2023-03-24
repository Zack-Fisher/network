use bevy::prelude::*;

use serde::{Serialize, Deserialize};
use bevy::utils::Uuid;

use crate::{inventory::ItemData, world_interaction::{collectible::{CollectibleBundle, Collectible}, analysis::AnalyseBundle}};

//primitive shapes to help with level design
#[derive(Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct CollectiblePrefab {
    pub range: f32,
    pub item: ItemData, 
    /// make a uuid, but super-secretly save it as a string.
    /// serializing the uuid itslef is probably safer, but would take
    /// silly things to make it work.
    pub uuid: String,
}

impl Default for CollectiblePrefab {
    fn default() -> Self {
        Self {
            range: f32::default(),
            item: ItemData::default(),
            uuid: Uuid::new_v4().to_string(),
        }
    }
}

pub fn build_collectible (
    mut commands: Commands,

    prefab_q: Query<(Entity, &CollectiblePrefab), Added<CollectiblePrefab>>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
)
{
    for (ent, col_prefab) in prefab_q.iter() {
        commands.entity(ent)
            .with_children(|children| {
                children
                    .spawn(
                        CollectibleBundle {
                            analyse: AnalyseBundle::default(),
                            collectible: Collectible {
                                // this is what makes everything work. 
                                // we need to pass the serialized identifier, rather
                                // than letting it generate its own thing.
                                range: col_prefab.range.clone(),
                                uuid: col_prefab.uuid.clone(),
                                item: col_prefab.item.clone(),
                                ..default()
                            },
                        }
                    )
                    ;
            });
    }
}
