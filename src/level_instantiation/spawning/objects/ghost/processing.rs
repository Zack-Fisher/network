use bevy::prelude::*;

use super::GhostPrefab;

pub fn ghost_process (
    mut ghost_q: Query<&GhostPrefab>,
)
{
    for ghost_comp in ghost_q.iter_mut() {
        
    }
}
