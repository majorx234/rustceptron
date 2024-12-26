use crate::layer::{activation_fct::ReLU, Layer};
use rand::Rng;

fn calc_cost(y1_vec: &[f32], y2_vec: &[f32]) -> f32 {
    assert_eq!(y1_vec.len(), y2_vec.len());
    let mut costs: f32 = 0.0;
    for (y1, y2) in y1_vec.iter().zip(y2_vec.iter()) {
        let diff = y1 - y2;
        costs += diff * diff;
    }
    costs
}

pub struct Network {
    pub layers: Vec<Layer>,
    pub activation_vecs: Vec<Vec<f32>>,
    pub bias: Vec<Vec<f32>>,
    pub step_count: u32,
}

impl Network {
    pub fn new(layer_sizes: Vec<(usize, usize)>) -> Self {
        let mut layers = Vec::new();
        let mut bias = Vec::new();

        for layer_size in &layer_sizes {
            layers.push(Layer::new(
                layer_size.0,
                layer_size.1,
                Box::new(ReLU::new(1.0)),
            ));
            bias.push(vec![0.0; layer_size.1]);
        }

        let mut activation_vecs: Vec<Vec<f32>> = Vec::new();
        for i in 1..layer_sizes.len() {
            let activation_fct: Vec<f32> = vec![0.0; layers[i].width];
            activation_vecs.push(activation_fct);
        }

        Network {
            layers,
            activation_vecs,
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
        let lower = 0.0;
        let upper = 1.0;
        for bias_vec in self.bias.iter_mut() {
            bias_vec.iter_mut().for_each(|x| {
                *x = rng.gen_range(lower..upper);
            });
        }
    }

    pub fn learn(&mut self, dataset: &[(&[f32], &[f32])]) {
        for (_idx, (input, output)) in dataset.iter().enumerate() {
            for (jdx, x) in self.activation_vecs[0].iter_mut().enumerate() {
                *x = input[jdx];
            }
            self.forward_step();
            let last_index = self.activation_vecs.len() - 1;
            let costs = calc_cost(&self.activation_vecs[last_index], &output);
        }
    }
    pub fn simple_forward(&mut self, input: Vec<f32>) {
        for (x, input) in self.activation_vecs[0].iter_mut().zip(input.iter()) {
            *x = *input;
        }
        self.forward_step();
    }

    fn forward_step(&mut self) {
        for i in 0..self.layers.len() {
            // multiplicate layer with input
            self.layers[i].vector_mul(&mut self.activation_vecs[i..i + 1]);
            // calc sum with bias vector
            self.activation_vecs[i]
                .iter_mut()
                .enumerate()
                .for_each(|(idx, x)| *x += self.bias[i][idx]);
            // calc activation function
            self.activation_vecs[i].iter_mut().for_each(|x| {
                *x = self.layers[i].activation_fct.fvalue(*x);
            });
        }
    }

    // for future printing
    fn investigat_network(
        &self,
        mut investigate_layer_fct: impl FnMut(usize, usize, usize, usize, usize, usize, f32) -> (),
        mut investigate_activation_vecs_fct: impl FnMut(usize, usize, usize, usize, f32) -> (),
        mut investigate_bias_fct: impl FnMut(usize, usize, usize, usize, f32) -> (),
    ) {
        // call investigate layer fct
        for layer_idx in 0..self.layers.len() {
            let height = self.layers[layer_idx].height;
            let width = self.layers[layer_idx].width;
            for i in 0..height {
                for j in 0..width {
                    investigate_layer_fct(
                        self.layers.len(),
                        layer_idx,
                        height,
                        width,
                        i,
                        j,
                        self.layers[layer_idx].data[j + i * width],
                    );
                }
            }
        }

        // investigate activation_vecs
        let nvec: usize = self.activation_vecs.len();
        for (idx, activation_vec) in self.activation_vecs.iter().enumerate() {
            let length: usize = activation_vec.len();
            for (jdx, activation) in activation_vec.iter().enumerate() {
                investigate_activation_vecs_fct(nvec, idx, length, jdx, *activation);
            }
        }

        //investigate biases
        let nbias_vecs: usize = self.bias.len();
        for (idx, bias_vec) in self.bias.iter().enumerate() {
            let length: usize = bias_vec.len();
            for (jdx, bias_value) in bias_vec.iter().enumerate() {
                investigate_bias_fct(nbias_vecs, idx, length, jdx, *bias_value);
            }
        }
    }
}
