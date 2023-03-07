use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::player_control::actions::UiAction;

pub struct MapUIPlugin;

impl Plugin for MapUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(build_mapui)

            //to be modified by the player and handled here.
            .insert_resource(MapHandle(None))

            .add_system(mapuibase_process)
            .add_system(map_image_process)
            ;
    }
}

#[derive(Resource)]
pub struct MapHandle(pub Option<Handle<Image>>);

#[derive(Component)]
struct MapUIBase;

#[derive(Component)]
struct MapImage;

fn build_mapui (
    mut commands: Commands,

    server: Res<AssetServer>,
)
{
    info!("building the map ui");
    commands
        .spawn(
            NodeBundle {
                background_color: BackgroundColor(Color::rgba(0.1, 0.1, 0.5, 0.5)),
                z_index: ZIndex::Global(5),
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                    ..default()
                },
                ..default()
            }
        )
        .with_children(|children| {
            children
                .spawn(
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection {
                                    value: String::from("the map"),
                                    style: TextStyle {
                                        font: server.load("fonts/roboto.ttf"),
                                        font_size: 50.0,
                                        color: Color::BEIGE,
                                    }
                                }
                            ],
                            ..default()
                        },
                        ..default()
                    }
                )
                ;

            children
                .spawn(
                    ImageBundle {
                        style: Style {
                            position_type: PositionType::Relative,
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            ..default()
                        },
                        ..default()
                    }
                )
                .insert(MapImage)
                ;

        })
        .insert(MapUIBase)
        ;
}

fn mapuibase_process (
    mut base_q: Query<&mut Visibility, With<MapUIBase>>,

    uiaction_q: Query<&ActionState<UiAction>>,
)
{
    for actions in uiaction_q.iter() {
        if actions.just_pressed(UiAction::ToggleMap) {
            for mut vis in base_q.iter_mut() {
                vis.is_visible = !vis.is_visible;    
            }
        }
    }
}

fn map_image_process (
    mut image_q: Query<&mut UiImage, With<MapImage>>,

    map_image: Res<MapHandle>,
)
{
    match map_image.0.clone() {
        Some(image_handle) => {
            for mut ui_image in image_q.iter_mut() {
                ui_image.0 = image_handle.clone();
            }
        }
        None => {

        }
    }
}