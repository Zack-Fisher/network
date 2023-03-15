use bevy::prelude::*;

pub mod post_spawn_modification;
pub mod animation_link;
pub mod objects;
pub mod builders;

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnEvent>()

            .add_plugin(objects::ObjectPlugin)
            .add_plugin(builders::BuildersPlugin)

            .add_system(spawn_process)

            .add_system(animation_link::link_animations)

            //all the different marker prefab processes
            ;
    }
}

pub struct SpawnEvent {
    pub path: String,
}

fn spawn_process (
    mut commands: Commands,

    mut spawn_evr: EventReader<SpawnEvent>,

    server: Res<AssetServer>,
)
{
    //really simple for now, should work out?
    //don't really need the whole async load garbage, just stop loading when ready.
    //maybe add all dyn ents to a vector, and poll that for loading process?
    //ideally, this shouldn't even need to be called. this is just for hot-loading entities and stuff
    //they should all be bundled with the level type.
    //everything is prefabs!!

    for ev in spawn_evr.iter() {
        commands
            .spawn(
                DynamicSceneBundle {
                    scene: server.load(ev.path.clone()),
                    ..default()
                }
            );
    }
}
