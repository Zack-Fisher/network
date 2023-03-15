use bevy::{prelude::*, utils::HashMap, gltf::Gltf};

use crate::file_system_interaction::level_serialization::{WorldLoadRequest, CurrentLevel};

pub struct LevelAnimPlugin;

impl Plugin for LevelAnimPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(LevelAnimHolder(HashMap::new()))

            .add_event::<LevelAnim>()

            .add_system(process)
            .add_system(update_resource_on_load)
            ;
    }
}

#[derive(Resource)]
pub struct LevelAnimHolder(HashMap<String, Handle<AnimationClip>>);

/// "play the level animation of this name."
/// i don't know how animations are going to import, maybe fuzzy find for the nearest name if an exact match isn't found?
#[derive(Debug, Clone)]
pub struct LevelAnim {
    pub name: String,
}

fn process (
    mut anim_evr: EventReader<LevelAnim>,

    anim_holder: Res<LevelAnimHolder>,
    curr_level: Res<CurrentLevel>,

    mut animplayer_q: Query<&mut AnimationPlayer>,
)
{
    for ev in anim_evr.iter() {
        info!("recieved request for a level animation: {:?}", ev.clone());
        if let Some(anim_clip) = anim_holder.0.get(&ev.name.clone()) {
            info!("found the animation in the anim holder");
            if let Ok(mut anim_player) = animplayer_q.get_mut(curr_level.eid.clone()) {
                info!("found the animplayer, playing level animation.");
                anim_player.play(anim_clip.clone());
            }
        }
    }
}

// optimize this with local variables so it only loads once, instead of polling.
//TODO
fn update_resource_on_load (
    mut anim_holder: ResMut<LevelAnimHolder>,
    curr_level: Res<CurrentLevel>,
    gltf: Res<Assets<Gltf>>,
)
{
    if let Some(scene) = gltf.get(&curr_level.glb.clone()) {
        anim_holder.0 = scene.named_animations.clone(); 
    }
}
