use bevy::prelude::*;

use crate::accessories::enums::{WristAcc, HatAcc};

use self::key::KeyItem;

pub mod key;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Inventory::default())
            ;
    }
}

/// use a general-purpose struct to attach arbitrary item data to collectible components.
pub enum ItemData {
    Hat(HatAcc),
    Wrist(WristAcc),
    Key(KeyItem),
}

#[derive(Resource, Default)]
pub struct Inventory {
    pub hat_vec: Vec<HatAcc>,    
    pub wrist_vec: Vec<WristAcc>,    
    pub key_vec: Vec<KeyItem>,    
}
