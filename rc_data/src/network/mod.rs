use crate::layer::{activation_fct::ReLU, Layer};
use rand::Rng;
pub struct Network {
    pub layers: Vec<Layer>,
    pub bias: Vec<Vec<f32>>,
    pub step_count: u32,
}

impl Network {
    pub fn new(layer_sizes: Vec<(usize, usize)>) -> Self {
        let mut layers = Vec::new();
        let mut bias = Vec::new();
        for layer_size in layer_sizes {
            layers.push(Layer::new(
                layer_size.0,
                layer_size.1,
                Box::new(ReLU::new(1.0)),
            ));
            bias.push(vec![0.0; layer_size.1]);
        }
        Network {
            layers,
            bias,
            step_count: 0,
        }
    }
    pub fn init(&mut self) {
        let mut rng = rand::thread_rng();
        for layer in self.layers.iter_mut() {
            layer.data.iter_mut().for_each(|x| {
                let lower = 0.0;
                let upper = 1.0;
                *x = rng.gen_range(lower..upper);
            });
        }
        for bias_vec in self.bias.iter_mut() {
            bias_vec.iter_mut().for_each(|x| {
                let lower = 0.0;
                let upper = 1.0;
                *x = rng.gen_range(lower..upper);
            });
        }
    }

    pub fn learn(&mut self, dataset: &[(&[f32], &[f32])]) {
        // preperation of cache datastructure
        let mut activation_vecs: Vec<Vec<f32>> = Vec::new();
        for i in 0..self.layers.len() {
            let activation_fct: Vec<f32> = vec![0.0; self.layers[i].width];
            activation_vecs.push(activation_fct);
        }
        for (_idx, (input, outut)) in dataset.iter().enumerate() {
            for (jdx, x) in activation_vecs[0].iter_mut().enumerate() {
                *x = input[jdx];
            }
            self.forward_step(&mut activation_vecs);
            //TODO calc cost function
        }
    }
    pub fn simple_forward(&mut self, input: Vec<f32>) {
        let mut activation_vecs: Vec<Vec<f32>> = Vec::new();

        // preperation of cache datastructure
        activation_vecs.push(input);
        for i in 1..self.layers.len() {
            let activation_fct: Vec<f32> = vec![0.0; self.layers[i].width];
            activation_vecs.push(activation_fct);
        }
        self.forward_step(&mut activation_vecs);
    }

    fn forward_step(&mut self, activation_vec: &mut Vec<Vec<f32>>) -> Vec<f32> {
        for i in 0..self.layers.len() {
            // multiplicate layer with input
            self.layers[i].vector_mul(&mut activation_vec[i..i + 1]);
            // calc sum with bias vector
            activation_vec[i]
                .iter_mut()
                .enumerate()
                .for_each(|(idx, x)| *x += self.bias[i][idx]);
            // calc activation function
            activation_vec[i].iter_mut().for_each(|x| {
                *x = self.layers[i].activation_fct.fvalue(*x);
            });
        }
        Vec::new()
    }
}
