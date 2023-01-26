use std::path::Path;

use bevy::prelude::*;

use anyhow::Result;
use tch::{nn, nn::Module, Device, Tensor};

use crate::*;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(make_tensor)
            .add_startup_system(create_tensor);
    }
}

fn make_tensor() {
    let vs = nn::VarStore::new(Device::Cpu);
    let net = nn::seq()
        .add(nn::linear(&vs.root(), 2, 2, Default::default()))
        //map each tensor in this input layer to the relu'd tensor.
        .add_fn(|xs| xs.relu())
        .add(nn::linear(&vs.root(), 2, 2, Default::default()));
    let x = Tensor::rand(&[2, 2], (tch::Kind::Float, tch::Device::Cpu));
    let y = net.forward(&x);

    Tensor::print(&x);
    Tensor::print(&y);
}

fn create_tensor() {
    let y = Tensor::zeros(&[2, 5], (tch::Kind::Float, tch::Device::Cpu));
    let z = Tensor::randn(&[2, 5], (tch::Kind::Float, tch::Device::Cpu));
    let a = Tensor::ones(&[2, 5], (tch::Kind::Float, tch::Device::Cpu));
    Tensor::print(&y);
    Tensor::print(&z);
    Tensor::print(&a);

    //getting random outputs from random inputs, this is just a basic linear network.
    // Create a new variable store
    //the varstore holds all of the parameters of the network
    let vs = nn::VarStore::new(Device::Cpu);

    // Define the neural network
    let fc = nn::linear(&vs.root(), 2, 3, Default::default());

    // Create an input tensor
    //pointers to arrays are just faster, we don't have to copy the entire array.
    let input = Tensor::randn(&[2], (tch::Kind::Float, vs.device()));

    // Apply the neural network to the input
    //the forward function is the forward pass of the network
    //it's defined in nn::Module, which is the base trait of all neural networks in torch
    let output = fc.forward(&input);

    Tensor::print(&output);
    Tensor::print(&input);
}

