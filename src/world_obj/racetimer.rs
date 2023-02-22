use bevy::{prelude::*};

use std::time::{SystemTime, Duration};

use crate::player::player_controller::Player;

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<FlagEvent>()
            .insert_resource(Race { state: RaceState::Pre })
            .add_system(timer_process)
            .add_system(flag_process)
            .add_system(race_process)
            ;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum RaceState {
    Pre,
    During,
    Finished,
}

//have a Race struct that manages individual races, and have pointers to flags and timers.
//flags are just components on arbitrary entities. flags are linked to races, and timers are linked to races.
//"if the Race has RaceState finished, delete self."
//manage local state the old fashioned way, with an enum.
//arbitrary amounts of races possible, just keep building ontop.
//how do we deal with the pointers, though? we can't just pass a typical & address ref, that isn't implemented for Components in bevy.

//reflect allows us to pass basic pointers of struct instances around.
#[derive(Component, Reflect, Copy, Clone, Resource)]
pub struct Race {
    state: RaceState,
}

//just have one active race for now.

pub fn build_race (
    mut commands: &mut Commands,
    mut mesh: &mut ResMut<Assets<Mesh>>,
    mut material: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    active_race: &mut ResMut<Race>,
)
{
    active_race.state = RaceState::During;

    //pass the same exact damn race pointer
    //lfg
    //
    let timer_ent = build_timer(commands, mesh, material, asset_server);
    let flag_ent = build_flag(commands, material, mesh);

    //let the race be a god object? why not let the timer and flag asynchronously poll the race?
    //how can we link the timer and flag with the same race? why are they even seperate?
    //we might have multiple flags per race. also, we want to add the timer to the ui, not just its own thing.
    //returning the e_id out of a prefab builder method is good practice i think.

    //consider returning Box<>es to the components themselves, instead?
}

pub struct RaceEvent {
    pub new_state: RaceState,
}

fn race_process(
    mut active_race: ResMut<Race>,
    mut race_evr: EventReader<RaceEvent>,
)
{
    for ev in race_evr.iter()
    {
        active_race.state = ev.new_state;
    }
}

//the Race pointer has the same lifetime as the struct itself.
#[derive(Component)]
struct TimerPrefab {
    //just keep accumulating the deltatime
    curr_time: f32,
    //when the process hears a flag "complete" event, flip this and stop updating the timer.
    is_complete: bool,
}

pub fn build_timer (
    mut commands: &mut Commands,
    mut mesh: &mut ResMut<Assets<Mesh>>,
    mut material: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
) -> Entity
{
    use crate::ui::main::*;

    let timer_e_id = commands
        .spawn((
            TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: String::from("timer"),
                            style: TextStyle {
                                font: asset_server.load("fonts/party.otf"),
                                font_size: 100.0,
                                color: Color::WHITE,
                            },
                            ..default()
                        }
                    ],
                    ..default()
                },
                ..default()
            },
            TimerPrefab {
                curr_time: 0.0,
                is_complete: false,
            },
            Name::new("timer"),
        )).id();

    
    
    timer_e_id
}

fn timer_process (
    //'static: just keep this alive the entire duration of the program.
    mut timer_q: Query<(&mut Text, &mut TimerPrefab)>,
    asset_server: Res<AssetServer>,
    mut flag_evr: EventReader<FlagEvent>,
    time: Res<Time>,
    active_race: Res<Race>,
)
{
    if active_race.state == RaceState::During {
        for (mut text_c, mut timer_c) in timer_q.iter_mut() {
            timer_c.curr_time += time.delta_seconds();

            text_c.sections = vec![
                TextSection {
                    value: format!("{}", timer_c.curr_time),
                    style: TextStyle {
                        font: asset_server.load("fonts/roboto.ttf"),
                        font_size: 100.0,
                        color: Color::rgba(0.5, 0.5, 0.5, 0.8),
                    }
                }
            ];
        }
    }
}

#[derive(Component)]
struct FlagPrefab {
    //we can basically use prefab structs to define export variables, just like in any other visual game engine.
    pub trigger_radius: f32,
}

pub fn build_flag(
    mut commands: &mut Commands,
    mut material: &mut ResMut<Assets<StandardMaterial>>,
    mut mesh: &mut ResMut<Assets<Mesh>>,
) -> Entity
{
    let mesh_handle = mesh.add(
        Mesh::from(shape::Cube { size: 2.0 })
    );

    let mat_handle = material.add(
        StandardMaterial {
            base_color: Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 },
            ..default()
        }
    );

    commands
        .spawn((
            PbrBundle {
                mesh: mesh_handle,
                material: mat_handle,
                ..default()
            },
            FlagPrefab {
                trigger_radius: 3.0,
            },
        )).id()
}

struct FlagEvent;

fn flag_process (
    flag_q: Query<(&GlobalTransform, &'static FlagPrefab)>,
    player_q: Query<&GlobalTransform, With<Player>>,

    mut race_evw: EventWriter<RaceEvent>,

    active_race: Res<Race>,
)
{
    if active_race.state == RaceState::During {
        let player_gtf = player_q.single();
        for (flag_gtf, flag_p) in flag_q.iter() {
            let distance = (player_gtf.translation() - flag_gtf.translation()).length();

            race_evw.send(RaceEvent { new_state: RaceState::Finished })
        }
    }
}