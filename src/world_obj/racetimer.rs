use bevy::{prelude::*};

use std::time::{SystemTime, Duration};

use crate::player::player_controller::Player;

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<FlagEvent>()
            .add_system(timer_process)
            .add_system(flag_process)
            ;
    }
}

#[derive(Component)]
struct Race {
    timer: Box<TimerPrefab>,
    flag: Box<FlagPrefab>,
}

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
    asset_server: &Res<AssetServer>
)
{
    commands
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
        ));
}

fn timer_process (
    mut timer_q: Query<(&mut Text, &mut TimerPrefab)>,
    asset_server: Res<AssetServer>,
    mut flag_evr: EventReader<FlagEvent>,
    time: Res<Time>,
)
{
    for (mut text_c, mut timer_c) in timer_q.iter_mut() {
        //just freeze processing entirely if the timer_c happens to flag completed
        if timer_c.is_complete {return;}

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

        for ev in flag_evr.iter() {
            //just consider any flag to be true, for now. how can we link a timer to a specific race?
            //consider making some sort of pointer structure. Like, init a timerPrefab with a smart pointer to a FlagPrefab or something.
            //that could be pretty slick, and it's what i was doing in godot anyway.
            timer_c.is_complete = true;
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
)
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
                trigger_radius: 3.0
            },
        ));
}

struct FlagEvent;

fn flag_process (
    flag_q: Query<(&GlobalTransform, &FlagPrefab)>,
    player_q: Query<&GlobalTransform, With<Player>>,

    mut flag_evw: EventWriter<FlagEvent>,
)
{
    let player_gtf = player_q.single();
    for (flag_gtf, flag_p) in flag_q.iter() {
        let distance = (player_gtf.translation() - flag_gtf.translation()).length();

        if distance <= flag_p.trigger_radius {
            println!("hitting");
            flag_evw.send(FlagEvent);
        }
    }
}