use bevy::{prelude::*, ecs::event::Event};

use crate::ui::main::*;

use std::time::{SystemTime, Duration};

use crate::player::player_controller::Player;

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Race {
                state: RaceState::Pre, 
                checkpoint_count: 0,
             })

            //prefab builder events
            .add_event::<TimerPrefab>()
            .add_event::<FlagPrefab>()
            .add_event::<RaceBuilderEvent>()

            //race mgmt event
            .add_event::<RaceEvent>()

            //prefab builders, read event lines
            .add_system(build_timer)
            .add_system(build_flag)
            .add_system(build_race)

            //prefab processes
            .add_system(timer_process)
            .add_system(flag_process)
            .add_system(race_process)
            
            //etc component procs
            .add_system(outline_process)
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

pub struct RaceBuilderEvent {
    pub state: RaceState,
    pub checkpoints: Vec<FlagPrefab>,
}

//reflect allows us to pass basic pointers of struct instances around.
//WAIT! just use the prefab itself as the event struct that we pass around!!!
#[derive(Component, Resource)]
pub struct Race {
    pub state: RaceState,
    pub checkpoint_count: usize,
}

//just have one active race for now.

//follow this event builder pattern for prefabs. can probably be automated later.
fn build_race (
    mut active_race: ResMut<Race>,

    //grab the evr to itself, just make one for each ev in self_evr.iter()
    mut self_evr: EventReader<RaceBuilderEvent>,

    mut flagp_evw: EventWriter<FlagPrefab>,
    mut timerp_evw: EventWriter<TimerPrefab>,
)
{
    for ev in self_evr.iter() {
        //copy the values into the active one
        active_race.state = ev.state;
        active_race.checkpoint_count = ev.checkpoints.len();

        for flag in ev.checkpoints.iter() {
            flagp_evw.send(flag.clone());
        }

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

    flag_q: Query<&FlagPrefab>,
)
{
    for ev in race_evr.iter()
    {
        active_race.state = ev.new_state;
    }

    let mut done = true;

    let mut amount_hit: usize = 0;

    let mut local_count: usize = 0;
    for checkpoint in flag_q.iter() {
        local_count += 1;

        if !checkpoint.hit {
            done = false;
        } else {
            amount_hit += 1;
        }
    }

    info!("{}", amount_hit);

    //because then, clearly the flags aren't readied up.
    if local_count < active_race.checkpoint_count {
        return;
    }
    
    if done {
        active_race.state = RaceState::Finished;
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

#[derive(Component, Copy, Clone, Debug)]
pub struct FlagPrefab {
    //we can basically use prefab structs to define export variables, just like in any other visual game engine.
    pub trigger_radius: f32,
    pub position: Transform,
    pub hit: bool,
}

#[derive(Component)]
pub struct Outline {
    rot_speed: f32,
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


        //the green little outside thing, make it transparent to show the radius. maybe add particles later.
        let sphere_handle = mesh.add(
            Mesh::from(shape::UVSphere {radius: ev.trigger_radius, sectors: 5, stacks: 5})
        );

        let sphere_mat_handle = material.add(
            StandardMaterial {
                alpha_mode: AlphaMode::Blend,
                base_color: Color::rgba(0.1, 0.8, 0.1, 0.2),
                ..default()
            }
        );

        commands
            .spawn((
                PbrBundle {
                    mesh: mesh_handle,
                    material: mat_handle,
                    transform: ev.position,
                    ..default()
                },
                //just pump the actual value here
                ev.clone(),
                Name::new("checkpoint"),
            ))
            ;

        commands
            .spawn(
                PbrBundle {
                    mesh: sphere_handle,
                    material: sphere_mat_handle,
                    transform: ev.position,
                    ..default()
                }
            )
            .insert(Outline {
                rot_speed: 1.0
            })
            ;
    }
}

fn flag_process (
    mut flag_q: Query<(&GlobalTransform, &mut FlagPrefab)>,
    player_q: Query<&GlobalTransform, With<Player>>,

    active_race: Res<Race>,
)
{
    if active_race.state == RaceState::During {
        let player_gtf = player_q.single();
        for (flag_gtf, mut flag_p) in flag_q.iter_mut() {
            info!("{:?}", flag_p);
            if flag_p.hit {
                return;
            }

            let distance = (player_gtf.translation() - flag_gtf.translation()).length();

            if distance <= flag_p.trigger_radius {
                flag_p.hit = true;
            }
        }
    }
}


// helper component processes

fn outline_process (
    mut outline_q: Query<(&mut Transform, &Outline)>,
)
{
    for (mut outline_tf, outline_c) in outline_q.iter_mut() {
        outline_tf.rotate_y(0.05 * outline_c.rot_speed);
        outline_tf.rotate_x(0.01 * outline_c.rot_speed);
    }
}