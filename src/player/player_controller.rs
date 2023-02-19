use bevy::core::Zeroable;
use bevy::prelude::*;
use bevy::input::keyboard::*;
use bevy::input::mouse::*;
use bevy::input::gamepad::*;
use bevy::input::Input;

use crate::input::*;

use bevy_rapier3d::prelude::*;

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
    pub gravity_scale: f32,
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
                    transform: Transform::from_xyz(0.0, 2.0, 3.0).looking_at([0.0, -0.3, -1.0].into(), Vec3::Y),
                    ..default()
                })
                .insert(PlayerCamera)
                .insert(Name::new("camera"));
            })
            .insert(Pivot)
            .insert(Name::new("camera pivot"));
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(0.5))
        .insert(KinematicCharacterController::default())
        .insert(GravityScale(2.0))
        .insert(ExternalImpulse {
            ..default()
        })
        .insert(Player { speed: 15.0, gravity_scale: 0.1 })
        .insert(Health { value: 100.0 });
}

fn move_player(
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut KinematicCharacterController, &Player)>,
    camera_query: Query<&GlobalTransform, (With<Pivot>, Without<Player>)>,
    time: Res<Time>,
    mapping: Res<InputMapping>,
) {
    let (mut player_kb, player_comp) = player_query.single_mut();
    let camera_tf = camera_query.single();

    let gravity_vec = Vec3::new(0.0, -1.0 * player_comp.gravity_scale, 0.0);

    //this is a built-in function on bevy transforms.
    let mut forward = camera_tf.forward();
    forward.y = 0.0;
    //make the vector all nice and unit-length
    forward = forward.normalize();

    let mut left = camera_tf.left();
    left.y = 0.0;
    left = left.normalize();

    let speed = player_comp.speed;

    let mut direction = Vec3::ZERO;

    //consider using a direct import if doing lots of conversions, more terse
    use crate::input::Action::*;

    if keyboard.pressed(get_key(&mapping, MoveUp)) {
        direction += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(get_key(&mapping, MoveDown)) {
        direction -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(get_key(&mapping, MoveLeft)) {
        direction += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(get_key(&mapping, MoveRight)) {
        direction -= left * time.delta_seconds() * speed;
    }

    if keyboard.pressed(get_key(&mapping, PlayerJump)) {
        direction += Vec3::new(0.0, 3.0, 0.0) * time.delta_seconds() * speed;
    }

    direction += gravity_vec;

    player_kb.translation = Some(direction);
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
