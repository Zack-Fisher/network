use bevy::prelude::*;

use serde::{Serialize, Deserialize};

use crate::player_control::player_embodiment::Player;

use super::{ActivateRace, RaceTable, Races};

//primitive shapes to help with level design
#[derive(Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct CheckpointPrefab {
    pub target_radius: f32,
    pub color: Color,
    pub race_name: Races,
    pub is_hit: bool,
}

impl Default for CheckpointPrefab {
    fn default() -> Self {
        Self {
            target_radius: 3.0,
            color: Color::rgba(1.0, 0.5, 0.3, 1.0),
            race_name: Races::default(),
            is_hit: false,
        }
    }
}

#[derive(Component)]
pub struct CheckpointOutline;

pub fn build_checkpoint (
    mut commands: Commands,

    prefab_q: Query<(Entity, &CheckpointPrefab), Added<CheckpointPrefab>>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
)
{
    for (ent, check_prefab) in prefab_q.iter() {
        commands.entity(ent)
            .with_children(|children| {
                children
                    .spawn(
                        SpatialBundle::default()
                    )
                    .with_children(|grandchildren| {
                        grandchildren
                            .spawn(
                                PbrBundle {
                                    mesh: meshes.add(Mesh::from(shape::UVSphere { radius: check_prefab.target_radius, ..default() })),
                                    material: mats.add(
                                        StandardMaterial {
                                            base_color: check_prefab.color,
                                            alpha_mode: AlphaMode::Blend,
                                            ..default()
                                        }
                                    ),
                                    ..default()
                                }
                            )
                            .insert(CheckpointOutline)
                            ;
                    })
                    ;
            });
    }
}

pub fn checkpoint_process (
    player_q: Query<&GlobalTransform, With<Player>>,
    mut checkpoint_q: Query<(&GlobalTransform, &mut CheckpointPrefab), Without<Player>>,

    mut racestart_evw: EventWriter<ActivateRace>,
    mut racetable: ResMut<RaceTable>,
)
{

    for player_gtf in player_q.iter() {
        for (checkpoint_gtf, mut checkpoint_prefab) in checkpoint_q.iter_mut() {
            let mut race_data = match racetable.table.get_mut(&format!("{:?}", checkpoint_prefab.race_name)) {
                Some(value) => {
                    if value.is_active {
                        //WHEN THE ASSOCIATED RACE IS ACTIVE, EXECUTE THIS
                    } else {
                        checkpoint_prefab.is_hit = false;
                    }

                    value
                },
                None => {
                    info!("this checkpoint doesn't correspond to any RaceData in the RaceTable resource. race_name: {:?}", checkpoint_prefab.race_name);
                    continue;
                }
            };

            if checkpoint_prefab.is_hit {
                continue;
            }

            if (player_gtf.translation() - checkpoint_gtf.translation()).length() < checkpoint_prefab.target_radius {
                info!("hitting the checkpoint");
                info!("this checkpoint's associated race: {:?}", checkpoint_prefab.race_name);

                checkpoint_prefab.is_hit = true;

                race_data.hit_count += 1;

                racestart_evw.send(ActivateRace {name: format!("{:?}", checkpoint_prefab.race_name)});
            }
        }
    }
}

pub fn checkpoint_outline_process (
    mut outline_q: Query<&mut Transform, With<CheckpointOutline>>,
)
{
    for mut outline_tf in outline_q.iter_mut() {
        outline_tf.rotate_x(0.01);
        outline_tf.rotate_y(0.01);
    }
}
