use bevy::prelude::*;
use crate::{level_instantiation::spawning::objects::npc::*, player_control::player_embodiment::Player};

///this manages all of the textbox components for npcs and stuff
///also handles the event system that shows text on individual textboxes.
pub struct TextboxPlugin;

impl Plugin for TextboxPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ShowText>()

            .add_system(show_text_process)
            .add_system(update_transform)
            ;        
    }
}

pub struct ShowText {
    pub char_entity: Entity,
    /// consider changing this to a vector of Strings? not sure what'll be necessary, so i shouldn't over-engineer this.
    pub text: String,
}

fn show_text_process (
    mut show_evr: EventReader<ShowText>,

    tboxlink_q: Query<&TextEntityLink>,

    mut text_q: Query<&mut Text, With<Textbox>>,

    server: Res<AssetServer>,
)
{
    for ev in show_evr.iter() {
        if let Ok(link_c) = tboxlink_q.get(ev.char_entity) {
            let tbox_ent = link_c.entity.clone(); 

            if let Ok(mut text_c) = text_q.get_mut(tbox_ent) {
                text_c.sections = vec![
                    TextSection {
                        value: ev.text.clone(),
                        style: TextStyle {
                            font: server.load("fonts/roboto.ttf"),
                            font_size: 50.0,
                            color: Color::BEIGE,
                        }
                    }
                ];

                info!("saying {} with entity {:?}", ev.text.clone(), ev.char_entity.clone());
            }
        }
    }
}

fn update_transform (
    //query for all transforms, then filter by e_id
    transform_q: Query<(&GlobalTransform, &TextEntityLink)>,

    mut textbox_q: Query<&mut Transform, With<Textbox>>,
)
{
    for (gtf, textlink_c) in transform_q.iter() {
        if let Ok(mut tbox_tf) = textbox_q.get_mut(textlink_c.entity) {
            //update translation. the transform is handled seperately.
            tbox_tf.translation = gtf.translation();
        }
    } 
}
