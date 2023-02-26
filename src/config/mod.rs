use bevy::prelude::*;

use bevy::ui::update;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<UpdateConfig>()
            .add_event::<SaveConfig>()

            .insert_resource(ConfigFile {
                bgm_volume: 0.1
            })

            .add_system(update_config)
            .add_system(save_config)
            ;
    }
}

#[derive(Resource, Serialize, Deserialize)]
pub struct ConfigFile {
    pub bgm_volume: f64,
}

//modify the config cleanly through event types.

pub struct UpdateConfig {
    pub bgm_volume: Option<f64>,
}

//set them all to none, it doesn't make sense to have defaults here.
//we just want to set only some values, not a uniform update on the config file
impl Default for UpdateConfig {
    fn default() -> UpdateConfig {
        UpdateConfig {
            bgm_volume: None,
        }
    }
}

fn update_config(
    mut config: ResMut<ConfigFile>,

    mut update_evr: EventReader<UpdateConfig>,
)
{
    for ev in update_evr.iter() {
        //slick!!! if it's None, set it to itself.
        config.bgm_volume = ev.bgm_volume.unwrap_or(config.bgm_volume);
    }
}

pub struct SaveConfig;

fn save_config(
    config: Res<ConfigFile>,

    mut save_evr: EventReader<SaveConfig>,
) 
{
    for ev in save_evr.iter() {
        let data = ConfigFile {
            bgm_volume: config.bgm_volume,
        };

        // Serialize the data to a file
        let path = Path::new("config.bin");

        let config_file = match File::create(&path){
            Ok(file) => {
                info!("created config file location.");
                file
            }
            Err(error) => {
                error!("failed to create config file! {}", error);
                return;
            }
        };

        let writer = BufWriter::new(config_file);
        match bincode::serialize_into(writer, &data) {
            Ok(()) => {
                info!("successfully serialized config file.");
            }
            Err(error) => {
                error!("failed to serialize the config file! {}", error);
            }
        };

        // Deserialize the data from the file
        let loaded_file = match File::open(&path) {
            Ok(result) => {
                info!("loaded the config file.");
                result
            }
            Err(error) => {
                error!("error! could not load the config file. {}", error);
                return;
            }
        };
        let reader = BufReader::new(loaded_file);
        let loaded_data: ConfigFile = match bincode::deserialize_from(reader) {
            Ok(data) => {
                info!("successfully deserialized the config file.");
                data
            }
            Err(error) => {
                error!("failed to deserialize the config file! {}", error);
                return;
            }
        };
    }
}
