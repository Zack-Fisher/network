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

use serde::{Serialize, Deserialize};

//huh
#[derive(Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
/// use a general-purpose struct to attach arbitrary item data to collectible components.
/// part of the Item wrapper struct.
/// we grab most of the Item-specific data from the actual variant attached to it and the implementations on that variant.
pub enum ItemVar {
    Hat(HatAcc),
    Wrist(WristAcc),
    Key(KeyItem),
}

impl Default for ItemVar {
    fn default() -> Self {
        Self::Hat(HatAcc::TopHat)
    }
}

//each Item enum that can be chosen in ItemVar should implement this.
trait Item {
    /// get the path of the model that represents the object.
    fn get_model_path(&self) -> &str;

    /// use format!() to grab the name of the variant using the Debug trait.
    fn get_name(&self) -> String;
}

#[derive(Resource, Default)]
pub struct Inventory {
    pub hat_vec: Vec<HatAcc>,
    pub wrist_vec: Vec<WristAcc>,
    pub key_vec: Vec<KeyItem>,
}

