use bevy::{prelude::*, utils::HashMap};

use crate::world_interaction::analysis::{Analysis, AnalysisEvent, event::EventTypes};

use super::objects::analysis::AnalysisPrefab;

/// there are, necessarily, a bunch of prefabs that will be loaded in based on the blender glb tagging system.
/// we'll let a "builder" system be the Added<Name> nametag handler systems.
/// these are specifically the ones that init various prefabs not associated with any system in particular.
pub struct BuildersPlugin;

impl Plugin for BuildersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(tag_handler)
            ;
    }
}

fn tag_handler (
    name_q: Query<(Entity, &Name), Added<Name>>,

    mut commands: Commands,
)
{
    for (ent, name) in name_q.iter() {
        let mut keyval_table: HashMap<String, String> = HashMap::new();

        let mut current_key = String::new();
        let mut current_value = String::new();
        let mut is_key = true;

        for c in name.chars() {
            if c == '[' {
                is_key = true;
            } else if c == '=' {
                is_key = false;
            } else if c == ']' {
                // Here you can decide what to do with each extracted key-value pair.
                // For this example, I'm just printing them out to the console.
                println!("{} = {}", current_key, current_value);

                keyval_table.insert(current_key.clone(), current_value.clone());

                current_key = String::new();
                current_value = String::new();
                is_key = true;
            } else {
                if is_key {
                    current_key.push(c);
                } else {
                    current_value.push(c);
                }
            }
        }

        let mut analysis = Analysis::default();

        for (key, value) in keyval_table.iter() {
            let mut event = AnalysisEvent::default();

            match key.as_str() {
                "door" => {
                    event.ev_type = EventTypes::Door;
                },
                "levelanim" => {
                    event.ev_type = EventTypes::LevelAnim;
                },
                //skip the loop, don't push a default event, might cause jank.
                _ => {continue;}
            }

            event.payload = value.clone();

            analysis.events.push(
                event
            );
        }

        if analysis.events.len() == 0 {
            info!("scanned the Name for any event tags, and none found. skipping... ({})", name);
            return;
        }

        info!("done scanning the Name, adding... ({}), ({:?})", name, analysis);

        commands.entity(ent)
            .insert(
                AnalysisPrefab {analysis}
            );
    }
}
