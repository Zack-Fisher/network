use bevy::prelude::*;

use serde::{Serialize, Deserialize};

//spawnpoints don't necessarily have any of their own stuff. they're just SpatialBundles with the components themselves stuck on.
//all that matters is their transform.
#[derive(Default, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct SpawnpointPrefab {
    //the name of this spawnpoint, targeted and searched for by the warppoint
    name: String,
}
