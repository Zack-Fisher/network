use bevy::prelude::*;
        .add_plugin(network::NetworkPlugin)

use tch::*;
use tch::nn::*;
use tch::Device;
use tch::Kind;


//through events, the world-controller module sends out periodic messages that dictate the behavior of the mover characters.
pub struct MoverMessage {
    pub direction: Vec2,
}

#[derive(Component)]
struct MoverNN;

pub struct WorldControllerPlugin;

impl Plugin for WorldControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MoverMessage>()
            .add_startup_system(init_mover_nn)
            .add_system(compute_mover_messages);
    }
}

fn init_mover_nn (
    mut commands: Commands,
) {
    //create the empty bundle that we'll attach the network to.
    commands
        .spawn(NodeBundle::default())
        .insert(MoverNN);
}

fn compute_mover_messages (
    mut message: EventWriter<MoverMessage>,
    mut network: Query<&mut MoverNN>,
) {
    let device = Device::Cpu;

    let vs = VarStore::new(device);

    //six inputs, eight hidden nodes, two outputs
    //we input the reward vec3 pos and the character vec3 pos, in that order, [reward: xyz -> player: xyz]
    let layer_one = linear(&vs.root(), 6, 8, Default::default());
    let layer_two = linear(&vs.root(), 8, 2, Default::default());

    // Create a new model, using the previously defined layers.

    let model = tch::nn::seq()
        .add(layer_one)
        .add_fn(|xs| xs.relu())
        .add(layer_two);

    // Define an example input tensor with shape (batch_size, 6)
    let input = tch::Tensor::rand(&[1, 6], (Kind::Float, Device::Cpu));

    // Forward the input through the model
    let output = model.forward(&input);

    let inner = output.get(0);
    let new_dir: Vec2 = Vec2::new(inner.get(0).into(), inner.get(1).into());

    //convert the output to a vec2 using the built in tensor methods

    message.send(MoverMessage {direction: new_dir})
}