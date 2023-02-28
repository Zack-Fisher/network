//management systems for the player's minimap.
use bevy::prelude::*;
use bevy_editor_pls::egui::Key;

use crate::input::InputMapping;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MapInitEvent>()

            .add_system(build_map)
            .add_system(map_process)
            ;
    }
}

//when the player's map camera is ready, it'll pass over the handle through an event to this module.
pub struct MapInitEvent {
    pub image_handle: Handle<Image>,
}

//marker
#[derive(Component)]
struct Map;

fn build_map(
    mut commands: Commands,
    map_q: Query<Entity, With<Map>>,

    mut mapinit_evr: EventReader<MapInitEvent>,
)
{
    for ev in mapinit_evr.iter() {
        let mut count: usize = 0;
        //there has to be a better way to get query counts
        for ent in map_q.iter() {
            count += 1;
        }

        //because then it already exists, and this is a redundant message.
        if count > 0 {
            info!("there's already a player map in the ui! returning...");
            return;
        }

        info!("spawning the player's map with the given Image Handle.");
        commands
            .spawn(
                NodeBundle {
                    background_color: BackgroundColor(Color::rgba(1.0, 0.2, 0.4, 0.2)),
                    z_index: ZIndex::Local(3),
                    ..default()
                }
            )
            .with_children(|children| {
                children
                    .spawn(
                ImageBundle {
                            image: UiImage(ev.image_handle.clone()),

                            z_index: ZIndex::Local(5), 
                            ..default()
                        }
                    )
                    .insert(Name::new("mapentity"))
                    ;
            })
            .insert(Map)
            .insert(Name::new("mapparentholder"))
            ;
    }
}

use crate::input::*;

fn map_process(
    mut map_q: Query<&mut Visibility, With<Map>>,

    keyboard: Res<Input<KeyCode>>,

    mapping: Res<InputMapping>,
)
{
    use crate::input::Action::*;

    if keyboard.pressed(get_key(&mapping, OpenMap)) {
        for mut vis_c in map_q.iter_mut() {
            vis_c.is_visible = !vis_c.is_visible;
        }
    }
}