use std::any::TypeId;

use bevy::prelude::*;

use super::*;

//this plugin builds accessory mounting points from model scene nametags.

pub struct BuilderPlugin;

impl Plugin for BuilderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(build_mounts)
            ;
    }
}

use crate::{util::get_top_character, movement::general_movement::Character};

//do a tag query.
//do all the queries for names in the tag builder all at once, faster?
// also setup the accessories elinks to the Character entity, for fast spawning.
// do it sooner rather rthan later, so that we don't have to search more than once.
//this is literally using the same exact function as animationlinks.
fn build_mounts (
    name_q: Query<(Entity, &Name), Added<Name>>,
    mut commands: Commands,

    mut acc_q: Query<&mut Accessories, With<Character>>,

    parent_q: Query<&Parent>,
    character_q: Query<&Character>,
)
{
    for (ent, name) in name_q.iter() {
        if name.to_lowercase().contains("[lwrist]") {
            establish_link::<LWrist>(&mut commands, &mut acc_q, &parent_q, &character_q, ent);
        }

        if name.to_lowercase().contains("[rwrist]") {
            establish_link::<RWrist>(&mut commands, &mut acc_q, &parent_q, &character_q, ent);
        }

        if name.to_lowercase().contains("[hat]") {
            establish_link::<Hat>(&mut commands, &mut acc_q, &parent_q, &character_q, ent);
        }
    }
}


fn establish_link<T> (
    mut commands: &mut Commands,

    mut acc_q: &mut Query<&mut Accessories, With<Character>>,

    parent_q: &Query<&Parent>,
    character_q: &Query<&Character>,

    ent: Entity,
)
{
    use std::any::type_name;

    let hat = type_name::<Hat>();
    let lwrist = type_name::<LWrist>();
    let rwrist = type_name::<RWrist>();


    let acc_holder_eid = commands.entity(ent)
        .with_children(
            |children| {
                let mut temp_eid = children
                    .spawn(
                        SpatialBundle::default() 
                    )
                    ;
                
                match type_name::<T>() {
                    _hat if _hat == hat => {
                        temp_eid.insert(Hat);
                    },
                    _lwrist if _lwrist == lwrist => {
                        temp_eid.insert(LWrist);
                    },
                    _rwrist if _rwrist == rwrist => {
                        temp_eid.insert(RWrist);
                    },
                    _ => {}
                }
            }
        )
        .id()
        ;

    let char_ent = get_top_character(acc_holder_eid, parent_q, character_q);

    if let Ok(mut char_acc) = acc_q.get_mut(char_ent) {

        match type_name::<T>() {
            _hat if _hat == hat => {
                char_acc.hat_ent = Some(acc_holder_eid.clone());
            },
            _lwrist if _lwrist == lwrist => {
                char_acc.l_wrist_ent = Some(acc_holder_eid.clone());
            },
            _rwrist if _rwrist == rwrist => {
                char_acc.r_wrist_ent = Some(acc_holder_eid.clone());
            },
            _ => {}
        }
    }
}
