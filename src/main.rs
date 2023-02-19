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
}
