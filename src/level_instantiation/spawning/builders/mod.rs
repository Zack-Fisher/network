use bevy::prelude::*;

use super::objects::button::ButtonPrefab;

/// there are, necessarily, a bunch of prefabs that will be loaded in based on the blender glb tagging system.
/// we'll let a "builder" system be the Added<Name> nametag handler systems.
/// these are specifically the ones that init various prefabs not associated with any system in particular.
pub struct BuildersPlugin;

impl Plugin for BuildersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(button_handler)
            ;
    }
}

fn button_handler (
    name_q: Query<(Entity, &Name), Added<Name>>,

    mut commands: Commands,
)
{
    for (ent, name) in name_q.iter() {
        if name.to_lowercase().trim().contains("[button=") {
            //grab the expected value from the tag, pop it into the payload
            let value = match name.split('=').nth(1) {
                Some(v) => v.trim_matches(|c| c == ']' || c == ' ').to_owned(),
                None => {
                    warn!("No value found after '=' sign in string: {}, spawning button with blank payload!!!", name);
                    String::new()
                }
            };

            info!("spawning button with payload {}", value.clone());

            commands.entity(ent)
                .insert(
                    ButtonPrefab {
                        payload: value.to_string().clone(),
                        ..default()
                    }
                );
            }
    }
}
