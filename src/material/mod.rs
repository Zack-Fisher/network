use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};

//the definition of custom material types bound with shaders. 

pub struct ShaderMaterialPlugin;

impl Plugin for ShaderMaterialPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(MaterialPlugin::<CustomMaterial>::default())
            ;
    }
}

//i think this is just about the minimal implementation you can make here.
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/test_frag.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

//remember, when loading a custom material query the World for ResMut<Assets<CustomMaterial>>, each has
//its own Assets frontend

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
//wtf
//there's a vscode extension to generate uuids automatically by command. check the wiki page on uuid/guid
#[uuid = "0122da1d-3ea4-48e4-8de2-8aed4b438d93"]
pub struct CustomMaterial {
    #[uniform(0)]
    pub color: Color,
    //apply both of these bindings to this one struct.
    //the struct forms our vertex layout that is passed to the shader itself.
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}