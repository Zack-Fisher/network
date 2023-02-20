use bevy::{prelude::{*, shape::Cube}, render::{render_resource::*, view::RenderLayers}};
use crate::character::dialogue::*;

pub struct NPCPlugin;

impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(manage_hoverbox);
    }
}

#[derive(Component)]
pub struct NPC {
    pub name: String,
}

//pass the commands to the function, not a direct system
pub fn build_npc(
    mut commands: &mut Commands,
    npc_data: NPC,
    //pull the asset server subset?
    //i guess it's specialized for each type it can load.
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
)
{
    commands
        .spawn(PbrBundle {
            //meshes.add() -> returns the Handle<> type for use in the pbrbundle.
            //it won't take the raw object, just the smart pointer.
            mesh: meshes.add(Mesh::from(Cube {size: 1.0})),
            material: materials.add(Color::rgb(0.5, 0.3, 0.6).into()),
            ..default()
        })
        .with_children(|children| {
            children
                //spbundle has transform and visibility, we'll hide the tbox when not using it
                .spawn(SpatialBundle {
                    ..default()
                })
                .insert(Hoverable::new(
                    String::from("dialogue is here"),
                    2.0
                ))
                .insert(Name::new(format!("{}_textboxholder", npc_data.name)));
        })
        .insert(Name::new(format!("{}", npc_data.name)));
}

#[derive(Component)]
struct TboxMesh;

pub fn build_tbox(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    mut images: &mut ResMut<Assets<Image>>,
)
{
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..default()
    };

    image.resize(size);

    //link the camera and ui nodes to the same render layer, to compartmentalize
    let first_pass_layer = RenderLayers::layer(1);

    let image_handle = images.add(image);

    //spawn the camera on the layer
    commands
        .spawn((Camera2dBundle {
            camera: Camera {
                priority: -1,
                target: bevy::render::camera::RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        },
        first_pass_layer
    ));

    //then, spawn the ui nodes that the camera will render
    commands
        .spawn((
            Text2dBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: String::from("textbox"),
                            ..default()
                        }
                    ],
                    ..default()
                },
                ..default()
            },
            first_pass_layer
        ));

        //not important for mvp right now
    // commands
    //     .spawn((
    //         SpriteBundle {
    //             texture: 
    //         }
    //     ))
    

    let cube_handle = meshes.add(Mesh::from(shape::Plane {size: 4.0}));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

    //now, actually spawn the textbox
    commands
        .spawn(
            PbrBundle {
                mesh: cube_handle,
                material: cube_material_handle,
                ..default()
            }
        );
}

fn manage_hoverbox (
    mut vis_query: Query<(&mut Visibility, &Hoverable)>,
)
{
    for (mut vis, hover_comp) in vis_query.iter_mut()
    {
        vis.is_visible = hover_comp.activated;
    }
}