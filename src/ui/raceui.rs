use bevy::prelude::*;

use iyes_loopless::prelude::*;

use crate::{level_instantiation::spawning::objects::race::{*, self}, ui::RACELAYER};

pub struct RaceUIPlugin;

impl Plugin for RaceUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(build_raceui)

            .add_system(raceui_process)
            .add_system(raceui_timer_process
                .run_if(is_racing)
            )
            .add_system(raceui_counter_process
                .run_if(is_racing)
            )
            ;        
    }
}

#[derive(Component)]
struct RaceUIBase;

#[derive(Component)]
struct RaceUITimer;

//the Counter shows the completion ratio of the race, how many checkpoints are left.
#[derive(Component)]
struct RaceUICounter;

fn build_raceui (
    mut commands: Commands,

    server: Res<AssetServer>,
)
{
    info!("building the race ui");
    commands
        .spawn(
            NodeBundle {
                background_color: BackgroundColor(Color::rgba(0.1, 0.1, 0.5, 0.5)),
                z_index: RACELAYER,
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                    ..default()
                },
                ..default()
            }
        )
        .with_children(|children| {
            children
                .spawn(
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection {
                                    value: String::from("timer"),
                                    style: TextStyle {
                                        font: server.load("fonts/roboto.ttf"),
                                        font_size: 50.0,
                                        color: Color::BEIGE,
                                    }
                                }
                            ],
                            ..default()
                        },
                        ..default()
                    }
                )
                .insert(RaceUITimer)
                ;

            children
                .spawn(
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection {
                                    value: String::from("counter"),
                                    style: TextStyle {
                                        font: server.load("fonts/roboto.ttf"),
                                        font_size: 50.0,
                                        color: Color::BEIGE,
                                    }
                                }
                            ],
                            ..default()
                        },
                        ..default()
                    }
                )
                .insert(RaceUICounter)
                ;
        })
        .insert(RaceUIBase)
        ;
}

fn raceui_process (
    mut raceui_q: Query<&mut Visibility, With<RaceUIBase>>,

    racetable: Res<RaceTable>,
)
{
    for mut visibility in raceui_q.iter_mut() {
        visibility.is_visible = racetable.is_racing.clone();
    }
}

fn raceui_counter_process (
    mut raceui_q: Query<&mut Text, With<RaceUICounter>>,

    racetable: Res<RaceTable>,

    server: Res<AssetServer>,
)
{
    for mut text in raceui_q.iter_mut() {
        text.sections = vec![];

        //unlock the data from the slice of our store (resource) with the bundled in key.
        match racetable.table.get(&racetable.active_race.clone()) {
            Some(value) => {
                text.sections.push(
                    TextSection {
                        value: String::from(format!("total: {} //// ", value.checkpoint_count)),
                        style: TextStyle {
                            font: server.load("fonts/roboto.ttf"),
                            font_size: 50.0,
                            color: Color::BEIGE,
                        }
                    }
                );
                text.sections.push(
                    TextSection {
                        value: String::from(format!("hit: {}", value.hit_count)),
                        style: TextStyle {
                            font: server.load("fonts/roboto.ttf"),
                            font_size: 50.0,
                            color: Color::BEIGE,
                        }
                    }
                );
            },
            None => {

            }
        }
    }
}

fn raceui_timer_process (
    mut raceui_q: Query<&mut Text, With<RaceUITimer>>,

    racetable: Res<RaceTable>,

    server: Res<AssetServer>,
)
{
    for mut text in raceui_q.iter_mut() {
        text.sections = vec![];

        match racetable.table.get(&racetable.active_race.clone()) {
            Some(value) => {
                text.sections.push(
                    TextSection {
                        value: String::from(format!("{}", value.curr_time)),
                        style: TextStyle {
                            font: server.load("fonts/roboto.ttf"),
                            font_size: 50.0,
                            color: Color::BEIGE,
                        }
                    }
                )
            },
            None => {

            }
        }
    }
}

//this isn't a normal system. it's an iyes loopless condition system.
fn is_racing (
    racetable: Res<RaceTable>,
) -> bool
{
    racetable.is_racing
}