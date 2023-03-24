use bevy::prelude::*;

use crate::{accessories::{Accessories, enums::HatAcc}, player_control::player_embodiment::Player};

use super::{EQUIPLAYER, UIState};

pub struct EquipPlugin;

impl Plugin for EquipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(build_equip_ui)
            
            .add_system(process)
            .add_system(equip_button_process)
            .add_system(equip_button_builder)
            ;
    }
}

#[derive(Component)]
pub struct EquipUIBase;

#[derive(Component)]
pub struct EquipButton;

fn build_equip_ui (
    mut commands: Commands,

    server: Res<AssetServer>,
)
{
    commands
        .spawn(
            NodeBundle {
                background_color: BackgroundColor(Color::rgba(0.1, 0.5, 0.3, 0.01)),
                z_index: EQUIPLAYER,
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
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgba(0.1, 0.1, 0.1, 0.5)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: server.load("fonts/roboto.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                })
                .insert(EquipButton)
                ;
        })
        .insert(EquipUIBase)
        ;
}

fn process (
    mut equip_q: Query<&mut Visibility, With<EquipUIBase>>,

    ui_state: Res<State<UIState>>,
)
{
    for mut vis in equip_q.iter_mut() {
        vis.is_visible = ui_state.current() == &UIState::EquipFocus;
    }
}

fn equip_button_process (
    equip_q: Query<&Interaction, (With<EquipButton>, Changed<Interaction>)>,

    //only modify the main Player's accessories.
    mut player_q: Query<&mut Accessories, With<Player>>,
)
{
    for int in equip_q.iter() {
        for mut acc in player_q.iter_mut() {
            match int.clone() {
                Interaction::Clicked => {
                    if let Some(_) = acc.hat {
                        acc.hat = None;
                    } else {
                        info!("equipping tophat");
                        acc.hat = Some(HatAcc::TopHat);
                    }
                }
                Interaction::Hovered => {
                }
                Interaction::None => {
                }
            }
        }
    }
}

/// how should we build buttons? how can we detect a change in a resource?
/// in godot, i used to just poll the data store for changes. right now, i think that's good enough for our resources.
/// another idea is to just use an immediate-mode style design, where it re-renders every frame. that's not going
/// to work well, probably.
fn equip_button_builder (
    //only modify the main Player's accessories.
    mut player_q: Query<&mut Accessories, With<Player>>,
)
{
    for int in equip_q.iter() {
        for mut acc in player_q.iter_mut() {
            match int.clone() {
                Interaction::Clicked => {
                    if let Some(_) = acc.hat {
                        acc.hat = None;
                    } else {
                        info!("equipping tophat");
                        acc.hat = Some(HatAcc::TopHat);
                    }
                }
                Interaction::Hovered => {
                }
                Interaction::None => {
                }
            }
        }
    }
}
