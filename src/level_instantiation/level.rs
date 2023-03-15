use bevy::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, strum_macros::EnumIter, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub enum Levels {
    #[default]
    Test,
    Testtwo,
    MVP,
    NAVMESHING,
}

impl Levels {
    pub fn to_string(&self) -> &str
    {
        match self {
            Self::Test => "levels/test/test.lvl.ron",
            Self::Testtwo => "levels/testtwo/testtwo.lvl.ron",
            Self::MVP => "levels/mvp/mvp.lvl.ron",
            Self::NAVMESHING => "levels/navmeshing/navmeshing.lvl.ron",
        }
    }
}