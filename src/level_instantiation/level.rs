use bevy::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, strum_macros::EnumIter)]
#[reflect(Component, Serialize, Deserialize)]
pub enum Levels {
    Test,
    Testtwo,
}

impl Default for Levels {
    fn default() -> Self
    {
        Self::Test
    }
}

impl Levels {
    pub fn to_string(&self) -> &str
    {
        match self {
            Self::Test => "levels/test/test.lvl.ron",
            Self::Testtwo => "levels/testtwo/testtwo.lvl.ron",
        }
    }
}