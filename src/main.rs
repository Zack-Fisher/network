use bevy::prelude::*;
use bevy::input::keyboard::*;
use bevy::input::mouse::*;

mod network;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "NETWORK".to_string(),
                        width: 500.0,
                        height: 500.0,
                        resizable: false,
                        ..Default::default()
                    },
                    ..default()
                    }
                )
        )
        .add_system(keyboard_input_system)
        .add_system(mouse_input_system)
        .run();
}

fn init_camera(
    mut commands: Commands,
) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn init_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        ..Default::default()
    });
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

