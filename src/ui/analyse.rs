use bevy::prelude::*;

use crate::world_interaction::analysis::CurrAnalysis;

use super::{ANALYSELAYER, UIState};

pub struct AnalysePlugin;

impl Plugin for AnalysePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(build_analyse_ui)

            .add_system(process)
            .add_system(analysis_process)
            ;
    }
}

#[derive(Component)]
pub struct Title;

#[derive(Component)]
pub struct AnalyseUIBase;

fn build_analyse_ui (
    mut commands: Commands,

    server: Res<AssetServer>,

)
{
    commands
        .spawn(
            NodeBundle {
                background_color: BackgroundColor(Color::rgba(0.9, 0.1, 0.1, 0.15)),
                z_index: ANALYSELAYER,
                style: Style { 
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..default()
                },
                ..default()
            }
        )
        .with_children(|children| {
            children
                .spawn(
                    NodeBundle {
                        background_color: BackgroundColor(Color::rgba(0.9, 0.1, 0.1, 0.15)),
                        style: Style { 
                            size: Size::new(Val::Percent(50.0), Val::Percent(90.0)),
                            ..default()
                        },
                        ..default()
                    }
                )
                .with_children(
                    |grandchildren| {
                        grandchildren
                            .spawn(
                                TextBundle {
                                    text: Text {
                                        sections: vec![
                                            TextSection {
                                                value: String::from("title"),
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
                            .insert(Title)
                            ;
                    }
                )
                ;
        })
        .insert(AnalyseUIBase)
        ;
}

fn process (
    mut equip_q: Query<&mut Visibility, With<AnalyseUIBase>>,
    mut curr_analysis: ResMut<CurrAnalysis>,

    ui_state: Res<State<UIState>>,

    mut local_vis: Local<bool>,
)
{
    for mut vis in equip_q.iter_mut() {
        if ui_state.current() == &UIState::AnalyseMode {
            if *local_vis == false {
                //a fresh change in state detected.

            }
            vis.is_visible = true;
            *local_vis = true;
        } else {
            if *local_vis == true {
                //a fresh change in state detected.
                //refresh the curr analysis.
                //maybe do this somewhere else, but this needs to know about ui state changes. 
                //it's not a single directional thing.
                curr_analysis.curr = None;
            }
            vis.is_visible = false;
            *local_vis = false;
        }
    }
}

fn analysis_process (
    curr_analysis: Res<CurrAnalysis>,

    mut title_q: Query<&mut Text, With<Title>>,

    server: Res<AssetServer>,
)
{
    if let Some(data) = curr_analysis.curr.clone() {
        for mut text in title_q.iter_mut() {
            text.sections = vec![
                TextSection {
                    value: data.title.clone(),
                    style: TextStyle {
                        font: server.load("fonts/roboto.ttf"),
                        font_size: 50.0,
                        color: Color::BEIGE,
                    }
                }
            ]
        }
    } else {
        //cleanup crew
        for mut text in title_q.iter_mut() {
            text.sections = vec![]
        }
    }
}
