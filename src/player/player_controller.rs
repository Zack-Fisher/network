use bevy::core::Zeroable;
use bevy::input::gamepad::*;
use bevy::input::keyboard::*;
use bevy::input::mouse::*;
use bevy::input::Input;
use bevy::prelude::*;

use crate::input::*;

use bevy_rapier3d::prelude::*;

//init the player_controller plugin
pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerDeathEvent>()
            .add_event::<PlayerHealthEvent>()
            .add_startup_system(build_player)
            .add_system(read_player_collisions)
            .add_system(move_camera)
            .add_system(move_player)
            ;
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub gravity_scale: f32,
    pub is_jumping: bool,
}

#[derive(Component)]
pub struct PlayerHealth {
    pub max_health: f32,
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
pub struct PlayerCamera {
    //the z-clamp params, in radians
    x_min: f32,
    x_max: f32,

    h_sensitivity: f32,
    v_sensitivity: f32,

    //invert the camera controls. 1.0 for normal, -1.0 for flipped.
    //just multiply by the mouse movement.
    h_flip: f32,
    v_flip: f32,
}

fn build_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
) {
    //make the player's base and mesh
    commands
        .spawn(SceneBundle {
            scene: server.load("models/character/Default.glb#Scene0"),
            transform: Transform::from_xyz(-4.0, 0.5, 4.0),
            ..default()
        })
        //have a pivot node for the third person camera
        .with_children(|spatial_parent| {
            spatial_parent
                .spawn(SpatialBundle { ..default() })
                .with_children(|parent| {
                    parent
                        .spawn(Camera3dBundle {
                            //by default, the mesh faces toward the negative z-axis
                            transform: Transform::from_xyz(0.0, 2.0, 3.0)
                                .looking_at([0.0, -0.3, -1.0].into(), Vec3::Y),
                            ..default()
                        })
                        .insert(PlayerCamera {
                            x_max: 0.25,
                            x_min: -0.25,
                            h_sensitivity: 0.3,
                            v_sensitivity: 0.3,
                            h_flip: -1.0,
                            v_flip: -1.0
                        })
                        .insert(Name::new("camera"));
                })
                .insert(Pivot)
                .insert(Name::new("camera pivot"));
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(0.5))
        .insert(KinematicCharacterController::default())
        .insert(GravityScale(2.0))
        .insert(ExternalImpulse { ..default() })
        .insert(Player {
            speed: 15.0,
            gravity_scale: 0.1,
            is_jumping: false,
        })
        .insert(PlayerHealth { max_health: 100.0, value: 100.0 })
        .insert(Name::new("skater boy"));
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

    if keyboard.pressed(get_key(&mapping, PlayerJump)) && !player_comp.is_jumping {
        direction += Vec3::new(0.0, 3.0, 0.0) * time.delta_seconds() * speed;
    }

    direction += gravity_vec;

    player_kb.translation = Some(direction);
}

//we rotate the pivot about the y-axis, and the camera up and down.
fn move_camera(
    keyboard: Res<Input<KeyCode>>,
    mut pivot_query: Query<&mut Transform, With<Pivot>>,
    mut camera_query: Query<(&mut Transform, &PlayerCamera), Without<Pivot>>,
    mut motion_evr: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    let mut pivot = pivot_query.single_mut();

    let (mut camera_tf, camera_comp) = camera_query.single_mut();

    let rotate_speed = 3.0;

    if keyboard.pressed(KeyCode::Q) {
        pivot.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds())
    }
    if keyboard.pressed(KeyCode::E) {
        pivot.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds())
    }

    for ev in motion_evr.iter() {
        pivot.rotate_axis(
            Vec3::Y,
            camera_comp.h_sensitivity * camera_comp.h_flip * time.delta_seconds() * ev.delta.x,
        );

        //rotate_axis is relative to parent transforms, which we don't want
        camera_tf.rotate_local_axis(
            Vec3::X,
            camera_comp.v_sensitivity * camera_comp.v_flip * time.delta_seconds() * ev.delta.y,
        );

        //apply a camera clamp
        let x_rotation = camera_tf.rotation.x;
        let clamped_x_rotation = x_rotation.max(camera_comp.x_max).min(camera_comp.x_min);
        camera_tf.rotation.x = clamped_x_rotation;
    }
}

fn read_player_collisions(
    mut character_controller_outputs: Query<(
        &mut KinematicCharacterControllerOutput,
        &mut Player,
        Entity,
    )>,
) {
    for (mut output, mut player, entity) in character_controller_outputs.iter_mut() {
        for collision in &output.collisions {
            set_jumping_false_if_touching_floor(entity, &mut player, collision);
        }
    }
}

fn set_jumping_false_if_touching_floor(
    entity: Entity,
    player: &mut Player,
    event: &CharacterCollision,
) {
    player.is_jumping = false;
}

#[derive(Reflect)]
pub enum PlayerHealthEventType {
    Damage,
    Healing,
}

pub struct PlayerHealthEvent {
    ev_type: PlayerHealthEventType,
    value: f32,
}

fn player_health_process (
    mut health_evr: EventReader<PlayerHealthEvent>,
    mut health_q: Query<&mut PlayerHealth, With<Player>>,

    mut death_evr: EventWriter<PlayerDeathEvent>,
)
{
    let mut health_c = health_q.single_mut();

    for ev in health_evr.iter() {
        match ev.ev_type {
            PlayerHealthEventType::Damage => {
                health_c.value -= ev.value;

                if health_c.value <= 0.0 {
                    death_evr.send(PlayerDeathEvent{});
                }
            }
            PlayerHealthEventType::Healing => {
                health_c.value += ev.value;

                if health_c.value >= health_c.max_health {
                    health_c.value = health_c.max_health;
                }
            }
        }
    }
}

pub struct PlayerDeathEvent {

}

fn player_death_process (
    mut death_evr: EventReader<PlayerDeathEvent>,
)
{
    for ev in death_evr.iter() {
        println!("dead event");
    }
}
