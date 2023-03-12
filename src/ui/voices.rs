use bevy::prelude::*;

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
)
{
    commands
        .spawn(
            NodeBundle {
                background_color: BackgroundColor(Color::rgba(0.1, 0.5, 0.3, 0.1)),
                z_index: VOICESLAYER,
                style: Style { 
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                    ..default()
                },
                ..default()
            }
        )
        .with_children(|children| {

        })
        .insert(VoicesUIBase)
        ;
}
