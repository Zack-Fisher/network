use bevy::prelude::*;
use bevy_editor_pls::egui::ProgressBar;
use bevy_gltf::Gltf;
use bevy_rapier3d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BuildCollisionEvent>()
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(setup_physics)
            .add_system(display_events);
    }
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(10.0, 0.1, 10.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}

pub struct BuildCollisionEvent {
    //load with the asset loader, pass to the build system.
    pub path: String,
}

//build and spawn the mesh with a pbrbundle and static rapier collision from the bevy Mesh.
//hopefully load this async on another thread, have a loading screen and only show when the whole level is properly initialized?
fn gen_collisions(
    mut commands: Commands,
    mut collbuild_evr: EventReader<BuildCollisionEvent>,
    server: Res<AssetServer>,
)
{
    for ev in collbuild_evr.iter() {
        let mesh: Handle<Mesh> = server.load(format!("{}{}", ev.path, "#Mesh0/Primitive0"));
    }
}

/* A system that displays the events. */
fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}