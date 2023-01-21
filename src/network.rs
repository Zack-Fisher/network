use tch::{nn, Device, Kind, Tensor};

pub fn create_model() -> nn::Sequential {
    nn::seq()
        .add(nn::conv2d(&[1, 28, 28], 8, 5, 1, 2))
        .add(nn::relu())
        .add(nn::max_pool2d(2, 2, 0))
        .add(nn::conv2d(8, 16, 5, 1, 2))
        .add(nn::relu())
        .add(nn::max_pool2d(2, 2, 0))
        .add(nn::flatten())
        .add(nn::linear(256, 10))
}
