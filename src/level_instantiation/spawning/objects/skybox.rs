use bevy::prelude::*;
//NOTE: PREFABS SHOULD ALWAYS BE ATTACHED TO SPATIALBUNDLES! THIS IS THE ASSUMPTION

use serde::{Serialize, Deserialize};

//primitive shapes to help with level design
#[derive(Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct SkyboxPrefab {
    pub image_path: String,
    pub x_rot: f32,
    pub y_rot: f32,
    pub z_rot: f32,
}

impl Default for SkyboxPrefab {
    fn default() -> Self {
        Self {
            image_path: "textures/skyboxes/testsky.png".to_string(),
            x_rot: 0.01,
            y_rot: 0.01,
            z_rot: 0.00,
        }
    }
}

pub fn build_skybox (
    mut commands: Commands,

    prefab_q: Query<(Entity, &SkyboxPrefab), Added<SkyboxPrefab>>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
)
{
    for (ent, skybox_prefab) in prefab_q.iter() {
        commands.entity(ent)
            .with_children(|children| {
                children
                    .spawn(
                        MaterialMeshBundle {
                            mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 150.0, ..default() })),
                            material: mats.add(StandardMaterial {
                                base_color_texture: Some(server.load(skybox_prefab.image_path.clone())),
                                ..default()
                            }),
                            ..default()
                        }
                    )
                    ;
            });
    }
}

pub fn skybox_process (
    mut skybox_q: Query<(&mut Transform, &SkyboxPrefab)>,
)
{
    for (mut sky_tf, sky_comp) in skybox_q.iter_mut() {
        sky_tf.rotate_x(sky_comp.x_rot);
        sky_tf.rotate_y(sky_comp.y_rot);
        sky_tf.rotate_z(sky_comp.z_rot);
    }
}
