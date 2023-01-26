use bevy::prelude::*;
use bevy::input::keyboard::*;
use bevy::input::mouse::*;
use bevy::input::gamepad::*;
use bevy::input::Input;

//init the player_controller plugin
pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
       app
        .add_startup_system(init_player)
        .add_system(move_player);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Health {
    pub value: f32,
}

#[derive(Component)]
pub struct Lifetime {
    pub value: f32,
}

//this is the heart of the first-person camera controller
//we initialize the camera and the player, the camera a child of the player mesh

fn init_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //make the player's base and mesh
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(-2.0, 0.5, 2.0),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                //by default, the mesh faces toward the negative z-axis
                transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at([0.0, 0.0, -1.0].into(), Vec3::Y),
                ..default()
            });
        })
        .insert(Player)
        .insert(Health { value: 100.0 });
}

fn move_player(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    //this is a built-in function on bevy transforms.
    let mut forward = camera.forward();
    forward.y = 0.0;
    //make the vector all nice and unit-length
    forward = forward.normalize();

    let mut left = camera.left();
    left.y = 0.0;
    left = left.normalize();

    let speed = 3.0;
    let rotate_speed = 3.0;

    if keyboard.pressed(KeyCode::W) {
        camera.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::A) {
        camera.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::D) {
        camera.translation -= left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds())
    }
    if keyboard.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds())
    }
}