use bevy::prelude::*;

use serde::{Serialize, Deserialize};
use bevy::utils::Uuid;

use crate::inventory::ItemData;

//primitive shapes to help with level design
#[derive(Default, Component, Reflect, Serialize, Deserialize)]
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
            });
    }
}
