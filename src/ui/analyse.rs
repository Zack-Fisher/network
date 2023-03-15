use bevy::prelude::*;

use super::ANALYSELAYER;

pub struct AnalysePlugin;

impl Plugin for AnalysePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(build_analyse_ui)
            ;
    }
}

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
                background_color: BackgroundColor(Color::rgba(0.1, 0.5, 0.3, 0.01)),
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
        })
        .insert(AnalyseUIBase)
        ;
}
