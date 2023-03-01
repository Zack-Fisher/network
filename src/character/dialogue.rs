use bevy::prelude::*;
use crate::player::player_controller::*;
use crate::character::npc::*;

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(compute_hoverable_text);        
    }
}

#[derive(Component)]
pub struct Hoverable {
    pub dialogue: String,
    pub activation_range: f32,
    pub activated: bool,
}

impl Default for Hoverable {
    fn default() -> Self {
        Hoverable {
            dialogue: String::from("default dialogue hover"),
            activation_range: 2.0,
            activated: false,
        }
    }
}

impl Hoverable {
    pub fn new(
        dialogue: String,
        activation_range: f32,
    ) -> Hoverable
    {
        Hoverable {
            dialogue,
            activation_range,
            activated: false,
        }
    }
}

fn compute_hoverable_text(
    mut commands: Commands,
    mut ent_query: Query<(&mut Hoverable, &GlobalTransform, Entity), Without<Player>>,
    player_query: Query<&GlobalTransform, With<Player>>,
)
{
    let player_tf = player_query.single();

    for (mut hoverable_comp, en_tf, entity) in ent_query.iter_mut()
    {
        let distance = (en_tf.translation() - player_tf.translation()).length();

        if distance <= hoverable_comp.activation_range
        {
            hoverable_comp.activated = true;
        } else {
            hoverable_comp.activated = false;
        }
    }
}
