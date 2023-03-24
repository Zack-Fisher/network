use bevy::prelude::*;

use crate::{accessories::enums::{WristAcc, HatAcc}, file_system_interaction::users::UserTable};

use self::key::KeyItem;

pub mod key;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<AddItem>()
            .add_event::<RemoveItem>()
            .add_event::<UseItem>()

            //don't set a resource here directly for the inventory. it's user-specific, so pull from the usertable
            //and the curruser.
            ;
    }
}

use serde::{Serialize, Deserialize};

//huh
#[derive(Clone, Serialize, Deserialize, Reflect)]
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

    /// run arbitrary code when the item is "used"
    fn use_item(&self);
}

//event lines to manage and read modifications to the currently active inventory.

///just pass a clone of the variant, and it'll search in the inventory to find it.
/// also, optionally pass the index in the Inventory vector that the item appears, if we need to operate on a specific one in the queue.
/// this really shouldn't matter for most use cases.
/// defaults to None, of course.
/// maybe that's stupid? i don't like the "ooh index of negative one if it doesn't exist"
/// i think that's even stupider than sucking it up and just using a fucking option
#[derive(Default)]
pub struct AddItem {
    pub item: ItemVar,
    pub index: Option<u32>,
}

#[derive(Default)]
pub struct UseItem {
    pub item: ItemVar,
    pub index: Option<u32>,
}

#[derive(Default)]
pub struct RemoveItem {
    pub item: ItemVar,
    pub index: Option<u32>,
}

fn handle_events (
    mut add_evr: EventReader<AddItem>,
    mut use_evr: EventReader<UseItem>,
    mut remove_evr: EventReader<RemoveItem>,

    mut usertable: ResMut<UserTable>,
)
{
    if let Some(name) = usertable.current_user {
        if let Some(mut user_data) = usertable.table.get_mut(&name) {
            for ev in add_evr.iter() {
                match ev.item.clone() {
                    ItemVar::Hat(hat) => {

                    },
                    ItemVar::Wrist(wrist) => {

                    },
                    ItemVar::Key(key) => {

                    }
                }
            }

            for ev in add_evr.iter() {
                match ev.item.clone() {
                    ItemVar::Hat(hat) => {

                    },
                    ItemVar::Wrist(wrist) => {

                    },
                    ItemVar::Key(key) => {

                    }
                }
            }

            for ev in add_evr.iter() {
                match ev.item.clone() {
                    ItemVar::Hat(hat) => {

                    },
                    ItemVar::Wrist(wrist) => {

                    },
                    ItemVar::Key(key) => {

                    }
                }
            }
        }
    }
}

#[derive(Resource, Default, Clone, Serialize, Deserialize)]
/// the idea here is that Vecs don't enforce uniqueness. we can have as many of an item as we want!
/// this is a system that's very flexible to serialization. It's much harder to break than storing all that data directly
/// in the pkv/the leveldata.
pub struct Inventory {
    pub hat_vec: Vec<HatAcc>,
    pub wrist_vec: Vec<WristAcc>,
    pub key_vec: Vec<KeyItem>,
}

