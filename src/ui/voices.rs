use bevy::prelude::*;

use crate::voices::VoicesTexture;

use super::VOICESLAYER;

pub struct VoicesPlugin;

impl Plugin for VoicesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(build_voices_ui)
            ;
    }
}

#[derive(Component)]
pub struct VoicesUIBase;

fn build_voices_ui (
    mut commands: Commands,

    server: Res<AssetServer>,

    voices_texture: Res<VoicesTexture>,
)
{
    commands
        .spawn(
            NodeBundle {
                background_color: BackgroundColor(Color::rgba(0.1, 0.5, 0.3, 0.01)),
                z_index: VOICESLAYER,
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
                    ImageBundle {
                        image: UiImage::from(voices_texture.0),
                        ..default()
                    }
                )
                ;
        })
        .insert(VoicesUIBase)
        ;
}
