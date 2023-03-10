use bevy::{prelude::*, utils::HashMap};

//keeps track of the current user and loads new ones, all from a resource.
pub struct UserPlugin;

impl Plugin for UserPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(UserTable {
                table: HashMap::new(),
                current_user: None,
            })

            .add_event::<LoadUser>()
            .add_event::<SaveUser>()

            .add_system(load_user_process)
            .add_system(save_user_process)


            .add_event::<SaveUserTable>()
            .add_event::<InitUserTable>()

            .add_system(save_user_table_process)
            .add_system(init_user_table_process)
            ;
    }
}

use bevy_pkv::PkvStore;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Resource)]
pub struct UserTable {
    /// the String in the HashMap should always just be the user's name.
    pub table: HashMap<String, UserAccount>,
    /// the same exact pattern as the RaceTable.
    /// put all required data in the same Resource data struct.
    /// use an Option type here instead of an is_active bool. cleaner (?)
    pub current_user: Option<String>,
}

/// the data structure that holds all of the user's stuff. to be loaded in the users.rs module and managed through CurrUser
#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize, Resource)]
#[reflect(Serialize, Deserialize)]
pub struct UserAccount {
    pub name: String,
    pub favorite_color: Color,
}

impl Default for UserAccount {
    fn default() -> Self {
        Self {
            name: String::from("JOHN EGBERT"),
            favorite_color: Color::rgb(0.4, 0.5, 0.1),
        }
    }
}

/// loads into the current_user in the usertable resource.
pub struct LoadUser {
    pub name: String,
}

/// saves the UserAccount into the UserTable resource.
/// overwrites if necessary.
/// names must be unique in the table, naturally.
pub struct SaveUser {
    pub user: UserAccount,
}

fn load_user_process (
    mut load_evr: EventReader<LoadUser>,
)
{
    for ev in load_evr.iter() {

    }
}

//save to a table, so that we can iterate through all the available users at any time.
//tables are more flexible than saving each file on its own in the pkv.
fn save_user_process (
    mut save_evr: EventReader<SaveUser>,

    mut pkv: ResMut<PkvStore>,
)
{
    for ev in save_evr.iter() {
        
    }   
}

const USERTABLE_KEY: &str = "usertable";

pub struct SaveUserTable;

pub struct InitUserTable;

fn save_user_table_process (
    mut pkv: ResMut<PkvStore>,
    mut user_table: ResMut<UserTable>,

    mut save_evr: EventReader<SaveUserTable>,
)
{
    for _ in save_evr.iter() {
        info!("saving the usertable to memory...");
        pkv.set::<HashMap<String, UserAccount>>(USERTABLE_KEY, &user_table.table);
    }
}

fn init_user_table_process (
    pkv: ResMut<PkvStore>,
    mut user_table: ResMut<UserTable>,

    mut init_evr: EventReader<InitUserTable>,
    mut save_evw: EventWriter<SaveUserTable>,
)
{
    for _ in init_evr.iter() {
        info!("initializing the usertable...");

        if let Ok(loaded_table) = pkv.get::<HashMap<String, UserAccount>>(USERTABLE_KEY) {
            info!("usertable in memory found! loading into the usertable resource...");
            user_table.table = loaded_table;
        } else {
            info!("usertable not found. creating new usertable...");
            user_table.table = HashMap::new();

            info!("usertable created. saving to memory...");
            save_evw.send(SaveUserTable);
        }
    }
}