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

//returns the first instance.
pub fn find_type_in_children<T: Component>(
    base_ent: Entity,

    child_q: &Query<&Children>,

    type_q: &Query<&T>,
) -> Option<Entity>
{
    if let Ok(children) = child_q.get(base_ent) {
        for child in children {
            if let Ok(_) = type_q.get(child.clone()) {
                return Some(child.clone());
            } else {
                if let Ok(grandchildren) = child_q.get(child.clone()) {
                    for grandchild in grandchildren {
                        match find_type_in_children::<T>(grandchild.clone(), child_q, type_q) {
                            Some(ent) => {return Some(ent)},
                            None => {},
                        }
                    }
                }
            }
        }
    }

    None
}

//returns all instances, in a Vector.
//jesus take the wheel 
pub fn find_all_of_type_in_children<T: Component>(
    base_ent: Entity,

    child_q: &Query<&Children>,

    type_q: &Query<&T>,
) -> Vec<Entity>
{
    let mut return_val: Vec<Entity> = Vec::new();

    if let Ok(children) = child_q.get(base_ent) {
        for child in children {
            if let Ok(_) = type_q.get(child.clone()) {
                return_val.push(child.clone());
            } else {
                if let Ok(grandchildren) = child_q.get(child.clone()) {
                    for grandchild in grandchildren {
                        match find_type_in_children::<T>(grandchild.clone(), child_q, type_q) {
                            Some(ent) => {return_val.push(ent)},
                            None => {},
                        }
                    }
                }
            }
        }
    }

    return_val
}