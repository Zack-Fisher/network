use bevy::{prelude::{*, shape::Cube}, render::{render_resource::*, view::RenderLayers}};
use crate::character::dialogue::*;

pub struct NPCPlugin;

impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NPCPrefab>()
            .add_system(manage_hoverbox);
    }
}

#[derive(Component, Clone)]
pub struct NPCPrefab {
    pub name: String,
}

#[derive(Component)]
struct TboxPrefab;

fn build_npc(
    mut commands: Commands,
    //pull the asset server subset?
    //i guess it's specialized for each type it can load.
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,

    mut npcb_evr: EventReader<NPCPrefab>,
)
{
    //first make the npc textbox
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

    let cube_handle = meshes.add(Mesh::from(shape::Plane {size: 4.0}));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

    for ev in npcb_evr.iter() {
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
                    .with_children(|grandchildren| {
                        
                        grandchildren
                            .spawn(
                                PbrBundle {
                                    mesh: cube_handle.clone(),
                                    material: cube_material_handle.clone(),
                                    ..default()
                                }
                            );
                    })
                    .insert(Hoverable::new(
                        String::from("dialogue is here"),
                        2.0
                    ))
                    .insert(Name::new(format!("{}_textboxholder", ev.name)));
            })
            .insert(ev.clone())
            .insert(Name::new(format!("{}", ev.name)));
    }
}


fn build_tbox(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
)
{
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