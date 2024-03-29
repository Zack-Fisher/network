use bevy::prelude::*;
use serde::{Serialize, Deserialize};

trait AccessoryItem {

}

#[derive(Clone, Copy, Eq, PartialEq, Hash, PartialOrd, FromReflect, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub enum WristAcc {
    Watch,
}

impl WristAcc {
    pub fn get_path(&self) -> String {
        let name: &str = match self {
            WristAcc::Watch => "wrist",
            _ => "error",
        };

        return format!("models/wrist/{}.glb#Scene0", name);
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Serialize, Deserialize, FromReflect, Reflect)]
#[reflect(Serialize, Deserialize)]
pub enum HatAcc {
    TopHat,
}

impl HatAcc {
    pub fn get_path(&self) -> String {
        let name: &str = match self {
            HatAcc::TopHat => "tophat",
            _ => "error",
        };

        return format!("models/hat/{}.glb#Scene0", name);
    }
}