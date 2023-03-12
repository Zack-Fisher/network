use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{player_control::actions::UiAction, chat::Messages};

use super::CHATLAYER;

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(build_chat_ui)

            .add_system(chat_ui_base_process)
            ;
    }
}

#[derive(Component)]
struct ChatUIBase;

#[derive(Component)]
struct MessageContainer;

fn build_chat_ui (
    mut commands: Commands,

    server: Res<AssetServer>,
)
{
    commands
        .spawn(
            NodeBundle {
                background_color: BackgroundColor(Color::rgba(0.1, 0.5, 0.3, 0.1)),
                z_index: CHATLAYER,
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
                    NodeBundle {
                        background_color: BackgroundColor(Color::rgba(0.1, 0.5, 0.3, 0.1)),
                        style: Style {
                            size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                            ..default()
                        },
                        ..default()
                    }
                )
                .with_children(|message_children| {
                    message_children
                        .spawn(
                            TextBundle {
                                text: Text {
                                    sections: vec![
                                        TextSection {
                                            value: String::from("message"),
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
                        .insert(MessageContainer)
                        ;
                })
                ;
        })
        .insert(ChatUIBase)
        ;
}

fn chat_ui_base_process (
    mut base_q: Query<&mut Visibility, With<ChatUIBase>>,

    uiaction_q: Query<&ActionState<UiAction>>,
)
{
    for actions in uiaction_q.iter() {
        if actions.just_pressed(UiAction::ToggleChat) {
            for mut vis in base_q.iter_mut() {
                vis.is_visible = !vis.is_visible;    
            }
        }
    }
}

fn chat_message_process (
    mut text_q: Query<&mut Text, With<MessageContainer>>,

    messages: Res<Messages>,

    server: Res<AssetServer>,
)
{
    for chat_text_comp in text_q.iter_mut() {
        let mut vec: Vec<TextSection> = vec![];

        for message in messages.vec.iter() {
            let message_body = format!(
                "{}: {}", message.name.clone(), message.text.clone()
            );

            vec.push(
                TextSection {
                    value: message_body.clone(),
                    style: TextStyle {
                        font: server.load("fonts/roboto.ttf"),
                        font_size: 50.0,
                        color: Color::BEIGE,
                    }
                }
            )
        }

        chat_text_comp.sections = vec;
    }
}
