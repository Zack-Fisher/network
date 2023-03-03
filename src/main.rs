use bevy::{prelude::*, ecs::world};
use bevy_editor_pls::{prelude::*, Editor};
use big_brain::prelude::*;
use bevy_egui::*;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod player;
mod ui;
mod character;
mod audio;
mod input;
mod utils;
mod physics;
mod world_obj;
mod config;
mod main_menu;
mod load;
mod material;

use material::CustomMaterial;
use player::player_controller::*;

use utils::three::*;

use bevy_framepace::FramepacePlugin;

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
        .add_state(GameState::MainMenu)

        .add_plugin(EditorPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        //limits to 60 by default.
        //settings.Limiter = Limiter::from_framerate(x), sets framerate to x.
        .add_plugin(BigBrainPlugin)
        .add_plugin(FramepacePlugin)
        .add_plugin(material::ShaderMaterialPlugin)
        .add_plugin(config::ConfigPlugin)
        .add_plugin(utils::DefaultUtilPlugin)
        .add_plugin(load::LoadPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(ui::UIPlugin)
        .add_plugin(world_obj::WorldObjectPlugin)
        .add_plugin(PlayerControllerPlugin)
        .add_plugin(character::CharacterPlugin)
        .add_plugin(audio::SoundPlugin)
        .add_plugin(physics::PhysicsPlugin)
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(TestingPlugin)

        .run();
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum GameState {
    MainMenu,
    Playing,
}

//put dummy systems for the purpose of testing new stuff, when I don't know where to put it.
pub struct TestingPlugin;

impl Plugin for TestingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(load_serialization_test)
            .add_startup_system(init_shader_test)
            .add_startup_system(init_scene)
            .add_startup_system(init_music)
            // .add_startup_system(init_race)
            // .add_startup_system_to_stage(StartupStage::PreStartup, init_glb_tester)
            .add_startup_system(init_npc)
            .add_startup_system(init_glb_scene)
            // .add_system(anim_glb)
            ;
    }
}

fn init_glb_scene (
    mut commands: Commands,

    server: Res<AssetServer>,
)
{
    commands
        .spawn(
            SceneBundle {
                scene: server.load("scenes/test.glb"),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            }
        )
        .insert(Name::new("try try again"))
        ;
}

use load::*;

fn load_serialization_test ()
{
    write_leveldata_to_file(
        LevelData {
            scene_path: String::from("models/level/leveltest.glb"),
            name: String::from("leveltest"),
        }, 
        String::from("test.level")
    );
}

fn init_shader_test(
    mut commands: Commands,

    mut meshes: ResMut<Assets<Mesh>>,
    mut mat: ResMut<Assets<CustomMaterial>>,

    server: Res<AssetServer>,
)
{
    commands
        .spawn(
            MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 3.0 })),
                material: mat.add(CustomMaterial {
                    color: Color::MIDNIGHT_BLUE,
                    color_texture: Some(server.load("textures/texture.png")),
                    alpha_mode: AlphaMode::Blend,
                }),
                transform: Transform::from_xyz(1.0, 4.0, 1.0),
                ..default()
            }
        )
        ;
}

use audio::bgm::*;
use config::*;

fn init_music(
    mut bgm_evw: EventWriter<PlayBGMEvent>,

    mut configupdate_evw: EventWriter<UpdateConfig>,
)
{
    configupdate_evw.send(
        UpdateConfig {
            bgm_volume: Some(0.5),
        }
    );
    info!("starting music");
    bgm_evw.send(
        PlayBGMEvent {
            speed: 1.5,
            ..default()
        }
    );
}

use character::npc::*;

fn init_npc(
    mut charb_evw: EventWriter<NPCPrefab>,
)
{
    charb_evw.send(
        NPCPrefab { name: String::from("john") }
    );
}

use world_obj::racetimer::*;

fn init_race(
    mut raceb_evw: EventWriter<RaceBuilderEvent>,
)
{
    raceb_evw.send(
        RaceBuilderEvent {
            state: RaceState::During,
            checkpoints: vec![
                FlagPrefab {
                    trigger_radius: 3.0,
                    position: Transform::from_xyz(1.0, 0.0, 5.0),
                    hit: false,
                },
                FlagPrefab {
                    trigger_radius: 5.0,
                    position: Transform::from_xyz(-4.0, 0.5, -3.0),
                    hit: false,
                },
            ]
        }
    )
}

//might have multiple animations on one glb.
#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

fn init_glb_tester(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
)
{
    commands.insert_resource(Animations(vec![
        asset_server.load("models/bouncy#Animation0")
    ]));

    let gltf_handle = asset_server.load("models/bouncy.glb#Scene0");
    commands
        .spawn(
            SceneBundle {
                scene: gltf_handle.clone(),
                transform: Transform::from_xyz(2.0, 1.0, 2.0),
                ..default()
            }
        );
}

//Local<T> is a local system variable, that gets passed between system calls. 
//neat.
fn anim_glb(
    mut player_q: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    mut done: Local<bool>,
)
{
    //reference Local system scoped variables with the * syntax. i guess it's passed around with a box pointer?
    if !*done {
        //if let scope!!!!
        //"do this if it returns a value." >> only for option types.
        for mut player in player_q.iter_mut() {
            //player the animation directly from the handle, from the asset server. "play" them on an AnimationPlayer component.
            //load the animation handle vec with .0 (loads as a tuple, i guess) then index the handle vector with [0]
            info!("play animation");
            info!("{:?}", animations.0);
            player.play(animations.0[0].clone_weak()).repeat();
            *done = true;
        }
    }
}

use bevy_rapier3d::prelude::*;

fn init_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // let scene_handle = asset_server.load("models/bumpymap_test.glb#Scene0");
    // let mesh: Handle<Mesh> = asset_server.load("models/bumpymap_test.glb#Mesh0/Primitive0");

    // let mut mesh_val = None;
    // while mesh_val.is_none() {
    //     match meshes.get(&mesh) {
    //         Some(val) => {
    //             mesh_val = Some(val);
    //         }
    //         None => {

    //         }
    //     }
    //     info!("looping");
    // }

    // commands
    //     .spawn(
    //         SceneBundle {
    //             scene: scene_handle,
    //             ..default()
    //         }
    //     )
    //     .insert(Collider::from_bevy_mesh(mesh_val.unwrap(), &ComputedColliderShape::TriMesh).unwrap());

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
