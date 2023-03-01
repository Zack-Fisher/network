use bevy::{prelude::*, log::Level};
use bevy_editor_pls::egui::ProgressBar;
use bevy_gltf::*;
use bevy_rapier3d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BuildLevelEvent>()

            .insert_resource(
                GLTFLoading(vec![]) 
            )
            .insert_resource(
                GLTFMeshLoading(vec![]) 
            )
            .insert_resource(
                MeshLoading(vec![]) 
            )

            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(setup_physics)
            .add_system(display_events)
            .add_system(gen_collisions)
            .add_system(check_gltf)
            .add_system(check_gltfmesh)
            .add_system(check_mesh)
            ;
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

pub struct BuildLevelEvent {
    //load with the asset loader, pass to the build system.
    pub path: String,
}

use bevy::tasks::*;

use crate::load::FinishLoad;

#[derive(Resource)]
struct GLTFLoading(Vec<Handle<Gltf>>);

#[derive(Resource)]
struct GLTFMeshLoading(Vec<Handle<GltfMesh>>);

#[derive(Resource)]
struct MeshLoading(Vec<Handle<Mesh>>);

#[derive(Component)]
struct LevelCollision;

#[derive(Component)]
struct LevelMesh;

//build and spawn the mesh with a pbrbundle and static rapier collision from the bevy Mesh.
//hopefully load this async on another thread, have a loading screen and only show when the whole level is properly initialized?
fn gen_collisions(
    mut commands: Commands,
    mut levelbuild_evr: EventReader<BuildLevelEvent>,
    mut finishbuild_evw: EventWriter<FinishLoad>,
    server: Res<AssetServer>,

    mut gltf_loading: ResMut<GLTFLoading>,
)
{
    for ev in levelbuild_evr.iter() {
        let level_gltf_handle: Handle<Gltf> = server.load(ev.path.clone());

        gltf_loading.0.push(level_gltf_handle);

        //also just load the mesh in a SceneBundle
        commands
            .spawn(
                SceneBundle {
                    scene: server.load(format!("{}#Scene0", ev.path.clone())),
                    ..default()
                }
            );
    }

}

use bevy::asset::LoadState;

fn check_gltf (
    server: Res<AssetServer>,
    mut gltf_loading: ResMut<GLTFLoading>,

    gltf: Res<Assets<Gltf>>,
    mut gltf_mesh_loading: ResMut<GLTFMeshLoading>,
)
{
    if gltf_loading.0.len() <= 0 {
        return;
    }
    match server.get_group_load_state(gltf_loading.0.iter().map(|h| h.id())) {
        LoadState::Failed => {
            error!("could not load the requested gltf for the level.");
        }
        LoadState::Loaded => {
            for gltf_handle in gltf_loading.0.clone().iter() {
                let my_gltf = gltf.get(&gltf_handle).unwrap();

                for gltf_mesh_handle in my_gltf.meshes.iter() {
                    gltf_mesh_loading.0.push(gltf_mesh_handle.clone());
                }

                gltf_loading.0 = vec![];
            }
        }
        _ => {

        }
    }
}

fn check_gltfmesh (
    mut commands: Commands,
    server: Res<AssetServer>,
    mut gltf_mesh_loading: ResMut<GLTFMeshLoading>,

    gltf_mesh: Res<Assets<GltfMesh>>,
    mut mesh_loading: ResMut<MeshLoading>,
)
{
    if gltf_mesh_loading.0.len() <= 0 {
        return;
    }
    match server.get_group_load_state(gltf_mesh_loading.0.iter().map(|h| h.id())) {
        LoadState::Failed => {
            error!("could not load the requested gltf for the level.");
        }
        LoadState::Loaded => {
            for gltf_mesh_handle in gltf_mesh_loading.0.clone().iter() {
                let my_gltf_mesh = gltf_mesh.get(&gltf_mesh_handle).unwrap();

                for gltf_mesh_prim in my_gltf_mesh.primitives.iter() {
                    mesh_loading.0.push(gltf_mesh_prim.mesh.clone());
                }
            }

            gltf_mesh_loading.0 = vec![];
        }
        _ => {
        }
    }
}

fn check_mesh (
    mut commands: Commands,
    server: Res<AssetServer>,
    mut mesh_loading: ResMut<MeshLoading>,

    mesh: Res<Assets<Mesh>>,

    mut finishl_evw: EventWriter<FinishLoad>,
)
{
    if mesh_loading.0.len() <= 0 {
        return;
    }

    match server.get_group_load_state(mesh_loading.0.iter().map(|h| h.id())) {
        LoadState::Failed => {
            error!("could not load the requested gltf for the level.");
        }
        LoadState::Loaded => {
            let mut spat_ent = commands
                .spawn(
                    SpatialBundle::default()
                )
                .insert(LevelCollision)
                .id()
                ;

            let mut builder_commands = commands.entity(spat_ent);

            for mesh_handle in mesh_loading.0.clone().iter() {
                let my_mesh = mesh.get(&mesh_handle).unwrap();

                builder_commands
                    .with_children(|children| {
                        children
                            .spawn(Collider::from_bevy_mesh(
                                my_mesh,
                                &ComputedColliderShape::TriMesh,
                            ).unwrap());
                    });
            }

            mesh_loading.0 = vec![];

            finishl_evw.send(
                FinishLoad
            );
        }
        _ => {
        }
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