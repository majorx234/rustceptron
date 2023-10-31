use crate::layer::{
    activation_fct::{ActivationFct, ReLU},
    Layer,
};
use num_traits::{Float, Inv, One, Zero};
use rand::{distributions::Uniform, prelude::Distribution, thread_rng, Rng};
pub struct Network<'a, T: Float> {
    pub layers: Vec<Layer<T>>,
    pub step_count: u32,
}

impl<T: Float + rand::distributions::uniform::SampleUniform> Network<'_, T> {
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
    pub fn init(&mut self) {
        let mut rng = rand::thread_rng();
        for layer in self.layers.iter_mut() {
            layer.data.iter_mut().for_each(|x| {
                let lower = Zero::zero();
                let upper = One::one();
                *x = rng.gen_range(lower..upper);
            });
        }
    }
}
