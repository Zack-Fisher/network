use bevy::prelude::*;

mod builders;
pub mod enums;

pub struct AccessoryPlugin;

impl Plugin for AccessoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(builders::BuilderPlugin)

            .add_system(accessory_process)
            ;
    }
}

use enums::*;

/// the main data structure, put onto the root of a Character and processed to show the accessories at the right places
#[derive(Component, Default)]
pub struct Accessories {
    pub l_wrist: Option<WristAcc>,
    pub l_wrist_ent: Option<Entity>,
    pub r_wrist: Option<WristAcc>,
    pub r_wrist_ent: Option<Entity>,
    pub hat: Option<HatAcc>,
    pub hat_ent: Option<Entity>,
}

// marker structs for where the scenebundles will go
#[derive(Component, Default, Eq, PartialEq)]
pub struct LWrist;

#[derive(Component, Default, Eq, PartialEq)]
pub struct RWrist;

#[derive(Component, Default, Eq, PartialEq)]
pub struct Hat;

//right now, a change in accessories will cause a full reload? does this matter?
//ghosts won't change accs that much anyway.
//anyway, worry about this if performance is bad.
fn accessory_process (
    acc_q: Query<(Entity, &Accessories), Changed<Accessories>>,

    server: Res<AssetServer>,

    mut commands: Commands,
)
{
    for (ent, acc) in acc_q.iter() {
        if let Some(l_wrist_enum) = acc.l_wrist {
            let acc_handle: Handle<Scene> = server.load(l_wrist_enum.get_path());
        }
        
        if let Some(hat_ent) = acc.hat_ent {
            if let Some(hat_enum) = acc.hat {
                let acc_handle: Handle<Scene> = server.load(hat_enum.get_path());

                commands.entity(hat_ent).despawn_descendants();

                commands.entity(hat_ent).with_children(
                    |children| {
                        children.spawn(
                            SceneBundle {
                                scene: acc_handle.clone(),
                                ..default()
                            }
                        );
                    }
                );
            } else {
                //if the hat_enum is None, just despawn the children.
                commands.entity(hat_ent).despawn_descendants();
            }
        }
    }
}
