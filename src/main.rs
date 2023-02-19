use bevy::prelude::*;
use bevy_editor_pls::{prelude::*, Editor};

mod player;
mod ui;
mod character;
mod audio;
mod input;
mod utils;
mod physics;

use player::player_controller::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::distributions::Standard;

use utils::three::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.9)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "NETWORK".to_string(),
                        width: 1280.0,
                        height: 720.0,
                        resizable: false,
                        ..Default::default()
                    },
                    ..default()
                    }
                )
        )
        .add_plugin(WorldInspectorPlugin)
        // .add_plugin(EditorPlugin)
        .add_plugin(utils::DefaultUtilPlugin)
        .add_plugin(input::InputPlugin)
        .add_startup_system(init_scene)
        .add_plugin(PlayerControllerPlugin)
        // .add_plugin(ui::main::MainUIPlugin)
        .add_plugin(character::CharacterPlugin)
        .add_plugin(audio::bgm::BGMPlugin)
        .add_plugin(physics::PhysicsPlugin)
        .run();
}


fn init_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(
            SceneBundle {
                scene: asset_server.load("models/shit_care.glb#Scene0"),
                ..default()
            }
        );

    //spawn a light

    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("Light"));

    commands
        .spawn(
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane {size: 1.0})),
                material: material.add(StandardMaterial {
                    base_color: Color::rgb(1.0, 0.0, 1.0),
                    ..default()
                }),
                ..default()
            }
        )
        .insert(Billboard::Y_BILL);
}
