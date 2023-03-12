use bevy::{prelude::*, render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}};

pub struct VoicesProcessingPlugin;

impl Plugin for VoicesProcessingPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(
                VoicesTexture::default()
            )

            .add_startup_system(init_texture)
            ;
    }
}

#[derive(Resource, Default)]
pub struct VoicesTexture(pub Handle<Image>);

fn init_texture (
    mut images: ResMut<Assets<Image>>,

    mut voices_texture: ResMut<VoicesTexture>,
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
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..default()
    };

    image.resize(size);

    let image_handle = images.add(image.clone());

    voices_texture.0 = image_handle.clone();
}
