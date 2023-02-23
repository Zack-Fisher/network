use bevy::{prelude::*, ecs::event::Event};

use crate::ui::main::*;

use std::time::{SystemTime, Duration};

use crate::player::player_controller::Player;

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Race { state: RaceState::Pre })
            .add_event::<TimerPrefab>()
            .add_event::<FlagPrefab>()
            .add_event::<Race>()
            .add_event::<RaceEvent>()
            .add_system(build_timer)
            .add_system(build_flag)
            .add_system(build_race)
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
//WAIT! just use the prefab itself as the event struct that we pass around!!!
#[derive(Component, Reflect, Copy, Clone, Resource)]
pub struct Race {
    pub state: RaceState,
}

//just have one active race for now.

//follow this event builder pattern for prefabs. can probably be automated later.
fn build_race (
    mut active_race: ResMut<Race>,

    //grab the evr to itself, just make one for each ev in self_evr.iter()
    mut self_evr: EventReader<Race>,

    mut flagp_evw: EventWriter<FlagPrefab>,
    mut timerp_evw: EventWriter<TimerPrefab>,
)
{
    for ev in self_evr.iter() {
        //copy the values into the active one
        active_race.state = ev.state;

        flagp_evw.send(FlagPrefab{ trigger_radius: 1.0 });
        //figure out a better way to skip initting curr_time to 0.0, this is clunky.
        timerp_evw.send(TimerPrefab { curr_time: 0.0, is_complete: false });
    }
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
#[derive(Component, Clone, Copy)]
pub struct TimerPrefab {
    //just keep accumulating the deltatime
    pub curr_time: f32,
    //when the process hears a flag "complete" event, flip this and stop updating the timer.
    pub is_complete: bool,
}

fn build_timer (
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
    mut ui_evw: EventWriter<UIAddEvent>,

    mut self_evr: EventReader<TimerPrefab>,
)
{
    for ev in self_evr.iter() {
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
                //the prefab needs to implement Copy and Clone, we just Copy the value over to the UI component then pass the ent.
                //doesn't need to be a pointer, just needs the right values in the right places.
                *ev,
                Name::new("timer"),
            )).id();

        //what is the most efficient way to pass the evwriter UIMain to here?
        
        //this does not displace the reference?
        //i guess Entity implements Copy?
        ui_evw.send(UIAddEvent {t: UIType::Timer, entity: timer_e_id});
    }
}

fn timer_process (
    //'static: just keep this alive the entire duration of the program.
    mut timer_q: Query<(&mut Text, &mut TimerPrefab)>,
    asset_server: Res<AssetServer>,
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

#[derive(Component, Copy, Clone)]
pub struct FlagPrefab {
    //we can basically use prefab structs to define export variables, just like in any other visual game engine.
    pub trigger_radius: f32,
}

//pass access to structs and event readers/writers, not functions.

fn build_flag(
    mut commands: Commands,
    mut material: ResMut<Assets<StandardMaterial>>,
    mut mesh: ResMut<Assets<Mesh>>,

    mut self_evr: EventReader<FlagPrefab>,
)
{
    for ev in self_evr.iter() {
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
                //just pump the actual value here
                //why does a deref not make rustc mad here lolol
                *ev,
            )).id();
    }
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

            if distance <= flag_p.trigger_radius {
                race_evw.send(RaceEvent { new_state: RaceState::Finished });
            }
        }
    }
}