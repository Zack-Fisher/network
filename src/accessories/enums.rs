use bevy::prelude::*;

#[derive(Clone, Copy, Eq, PartialEq, Hash, PartialOrd)]
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

#[derive(Clone, Copy, Eq, PartialEq, Hash, PartialOrd)]
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