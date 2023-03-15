use bevy::prelude::*;

use bevy_rapier3d::prelude::Collider;
use serde::{Serialize, Deserialize};

use crate::world_interaction::analysis::{AnalyseBundle, Analysis, AnalysisEventType, AnalysisEvent, event::EventTypes};

//primitive shapes to help with level design
#[derive(Default, Component, Reflect, Serialize, Deserialize, Debug)]
#[reflect(Component, Serialize, Deserialize)]
pub struct AnalysisPrefab {
    pub analysis: Analysis
}

pub fn build_analysis (
    mut commands: Commands,

    prefab_q: Query<(Entity, &AnalysisPrefab), Added<AnalysisPrefab>>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
)
{
    for (ent, prefab) in prefab_q.iter() {
        info!("spawning the button prefab from its children {:?}", prefab.clone());

        commands.entity(ent)
            .with_children(|children| {
                children
                    .spawn(
                        AnalyseBundle {
                            pbr: PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Cube {size: 3.0})),
                                material: mats.add(StandardMaterial {
                                    base_color: Color::rgba(0.5, 0.0, 0.1, 0.3),
                                    alpha_mode: AlphaMode::Blend,
                                    ..default()
                                }),
                                ..default()
                            },
                            analysis: Analysis(Some(AnalysisEvent {
                                    payload: prefab.payload.clone(),
                                    ev_type: AnalysisEventType::Button,
                                })),
                            ..default()
                        }
                    )
                    ;
            });
    }
}
