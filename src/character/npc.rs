use bevy::prelude::{*, shape::Cube};
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

fn manage_hoverbox (
    mut vis_query: Query<(&mut Visibility, &Hoverable)>,
)
{
    for (mut vis, hover_comp) in vis_query.iter_mut()
    {
        vis.is_visible = hover_comp.activated;
    }
}