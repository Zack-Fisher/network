use bevy::prelude::*;
use bevy::input::keyboard::*;
use bevy::input::mouse::*;

mod player;

use player::player_controller::*;

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
        .add_startup_system(init_scene)
        .add_plugin(PlayerControllerPlugin)
        .run();
}


fn init_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
        material: material.add(Color::rgb(0.9, 0.7, 0.3).into()),
        transform: Transform::from_xyz(0.0, -0.5, 0.0),
        ..Default::default()
    });

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

    //spawn a cube mesh
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
            material: material.add(Color::rgb(0.8, 0.7, 0.8).into()),
            transform: Transform::from_xyz(10.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(Name::new("Cube"));
}


//pass the event reader as a system parameter
fn keyboard_input_system(
    mut key_evr: EventReader<KeyboardInput>,
) {
    use bevy::input::ButtonState;

    for ev in key_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                println!("Key press: {:?} ({})", ev.key_code, ev.scan_code);
            }
            ButtonState::Released => {
                println!("Key release: {:?} ({})", ev.key_code, ev.scan_code);
            }
        }
    }
}

fn mouse_input_system(
    mut mouse_evr: EventReader<MouseButtonInput>,
) {
    use bevy::input::ButtonState;

    for ev in mouse_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                println!("Mouse press: {:?}", ev.button);
            }
            ButtonState::Released => {
                println!("Mouse release: {:?}", ev.button);
            }
        }
    }
}

