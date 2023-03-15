use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::movement::general_movement::Character;

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct AnimationEntityLink(pub Entity);

impl Default for AnimationEntityLink {
    fn default() -> Self {
        Self(Entity::from_raw(0))
    }
}

use crate::util::*;

pub fn link_animations(
    player_query: Query<Entity, Added<AnimationPlayer>>,
    parent_query: Query<&Parent>,
    character_query: Query<&Character>,
    animations_entity_link_query: Query<&AnimationEntityLink>,
    mut commands: Commands,
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("link_animations").entered();
    for entity in player_query.iter() {
        info!("creating the ael for the added animationplayer, from entity {:?}", entity.clone());
        let top_entity = get_top_character(entity, &parent_query, &character_query);
        if animations_entity_link_query.get(top_entity).is_ok() {
            warn!("Multiple `AnimationPlayer`s are ambiguous for the same top parent");
        } else {
            commands
                .entity(top_entity)
                .insert(AnimationEntityLink(entity));
        }
    }
}
