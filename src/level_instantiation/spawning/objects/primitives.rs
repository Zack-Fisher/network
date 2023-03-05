use bevy::prelude::*;

use bevy_rapier3d::prelude::Collider;
use serde::{Serialize, Deserialize};

//primitive shapes to help with level design
#[derive(Default, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct CubePrefab {
    pub size: f32,
    pub color: Color,
}

pub fn build_cube (
    mut commands: Commands,

    prefab_q: Query<(Entity, &CubePrefab), Added<CubePrefab>>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
)
{
    for (ent, cube_prefab) in prefab_q.iter() {
        commands.entity(ent)
            .with_children(|children| {
                children
                    .spawn(
                        PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: cube_prefab.size })),
                            material: mats.add(
                                StandardMaterial {
                                    base_color: cube_prefab.color,
                                    ..default()
                                }
                            ),
                            ..default()
                        }
                    )
                    .insert(Collider::cuboid(cube_prefab.size, cube_prefab.size, cube_prefab.size))
                    ;
            });
    }
}
