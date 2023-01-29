use bevy::prelude::*;

pub struct LoadPlugin;

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(load_scene);     
    }
}

struct Scene {
    entities: Vec<Entity>,
}

fn load_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scenes: ResMut<Assets<Scene>>,
    mut current_scene: ResMut<Option<Scene>>,
    scene_handle: Handle<Scene>,
) {
    // clear the current scene
    if let Some(mut current_scene) = current_scene.take() {
        for entity in current_scene.entities {
            commands.despawn(entity);
        }
    }
    // load the new scene
    let new_scene = scenes.get(scene_handle).unwrap();
    for entity in new_scene.entities.clone() {
        commands.spawn(entity);
    }
    current_scene.replace(new_scene);
}
