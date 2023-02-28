use bevy::prelude::*;

use crate::load::*;

pub struct LoadScreen;

impl Plugin for LoadScreen {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(LoadState::InitLoad)
                    .with_system(build_load_screen)
            )
            .add_system_set(
                SystemSet::on_enter(LoadState::Playing)
                    .with_system(destroy_load_screen)
            )
            ;
    }
}

#[derive(Component)]
struct LoadScreenMarker;

fn build_load_screen (
    mut commands: Commands,

    server: Res<AssetServer>,
)
{
    commands
        .spawn(
            NodeBundle {
                background_color: BackgroundColor(Color::rgba(0.2, 0.2, 0.2, 1.0)),
                ..default()
            }
        )
        .with_children(|children| {
            children
                .spawn(
                    TextBundle {
                        text: 
                            Text {
                                sections: vec![
                                    TextSection {
                                        value: String::from("loading"),
                                        style: TextStyle {
                                            font: server.load("fonts/roboto.ttf"),
                                            font_size: 100.0,
                                            color: Color::rgba(0.8, 0.8, 0.2, 0.6),
                                        },
                                        ..default()
                                    },
                                ],
                                ..default()
                            },
                        ..default()
                    }
                );
        })
        .insert(
            LoadScreenMarker
        )
        .insert(Name::new("loading screen main"))
        ;
}

fn destroy_load_screen (
    mut commands: Commands,
    load_q: Query<Entity, With<LoadScreenMarker>>,
)
{
    for load_ent in load_q.iter() {
        commands.entity(load_ent).despawn_recursive();
    }
}
