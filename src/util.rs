pub mod log_error;
pub mod trait_extension;

use bevy::prelude::*;

use crate::movement::general_movement::Character;

/// Source: <https://github.com/bevyengine/bevy/discussions/5564>
/// we actually don't want it to grab the top, just the highest character.
/// this is changed from the reference.
pub fn get_top_character(curr_entity: Entity, parent_query: &Query<&Parent>, character_query: &Query<&Character>) -> Entity {
    let mut last_entity = curr_entity;
    while let Ok(parent) = parent_query.get(last_entity) {
        last_entity = parent.get();
        // if there is a character component on the entity, return. this is the top level character.
        if let Ok(_) = character_query.get(last_entity) {
            return last_entity;
        }
    }
    last_entity
}
