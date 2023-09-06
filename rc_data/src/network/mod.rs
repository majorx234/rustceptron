use crate::layer::Layer;
use num_traits::{Float, Zero};

pub struct Network<T: Float> {
    pub layers: Vec<Layer<T>>,
    pub step_count: u32,
}

impl<T: Float> Network<T> {
    pub fn new(layer_sizes: Vec<(usize, usize)>) -> Self {
        let mut layers = Vec::new();
        for layer_size in layer_sizes {
            layers.push(Layer::<T>::new(layer_size.0, layer_size.1));
        }
        Network {
            layers,
            step_count: 0,
        }
    }
}
