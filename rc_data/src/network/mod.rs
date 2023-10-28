use crate::layer::{
    activation_fct::{ActivationFct, ReLU},
    Layer,
};
use num_traits::{Float, Zero};

pub struct Network<'a, T: Float> {
    pub layers: Vec<Layer<'a, T>>,
    pub step_count: u32,
}

impl<T: Float> Network<'_, T> {
    pub fn new(layer_sizes: Vec<(usize, usize)>) -> Self {
        let mut layers = Vec::new();
        for layer_size in layer_sizes {
            layers.push(Layer::<T>::new(
                layer_size.0,
                layer_size.1,
                Box::new(ReLU::new(1.0)),
            ));
        }
        Network {
            layers,
            step_count: 0,
        }
    }
}
