use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, FromReflect, Reflect)]
#[reflect(Serialize, Deserialize)]
pub enum KeyItem {

}