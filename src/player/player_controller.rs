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
        .add_system(move_camera)
        .add_system(move_player);
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

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

//marker for camera pivot
#[derive(Component)]
pub struct Pivot;

#[derive(Component)]
struct PlayerCamera;

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
        //have a pivot node for the third person camera
        .with_children(|spatial_parent| {
            spatial_parent.spawn(SpatialBundle {
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(Camera3dBundle {
                    //by default, the mesh faces toward the negative z-axis
                    transform: Transform::from_xyz(0.0, 0.0, 3.0).looking_at([0.0, 0.0, -1.0].into(), Vec3::Y),
                    ..default()
                })
                .insert(PlayerCamera)
                .insert(Name::new("camera"));
            })
            .insert(Pivot)
            .insert(Name::new("camera pivot"));
        })
        .insert(Player { speed: 5.0 })
        .insert(Health { value: 100.0 });
}

fn move_player(
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Player)>,
    camera_query: Query<&GlobalTransform, (With<Pivot>, Without<Player>)>,
    time: Res<Time>,
) {
    let (mut player, player_comp) = player_query.single_mut();
    let camera_tf = camera_query.single();

    //this is a built-in function on bevy transforms.
    let mut forward = camera_tf.forward();
    forward.y = 0.0;
    //make the vector all nice and unit-length
    forward = forward.normalize();

    let mut left = camera_tf.left();
    left.y = 0.0;
    left = left.normalize();

    let speed = player_comp.speed;
    let rotate_speed = 3.0;

    if keyboard.pressed(KeyCode::W) {
        player.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        player.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::A) {
        player.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::D) {
        player.translation -= left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::Q) {
        player.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds())
    }
    if keyboard.pressed(KeyCode::E) {
        player.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds())
    }
}

fn move_camera(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Pivot>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    let rotate_speed = 3.0;

    if keyboard.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds())
    }
    if keyboard.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds())
    }
}