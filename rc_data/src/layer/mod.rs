pub mod activation_fct;
use activation_fct::ActivationFct;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Layer {
    pub width: usize,
    pub height: usize,
    pub data: Vec<f32>,
    pub activation_fct: Box<dyn ActivationFct + 'static>,
}

impl Layer {
    pub fn new(width: usize, height: usize, activation_fct: Box<dyn ActivationFct>) -> Self {
        Self {
            width,
            height,
            data: vec![0.0; width * height + 1],
            activation_fct,
        }
    }
    pub fn clear(&mut self) {
        self.data.iter_mut().for_each(|x| {
            *x = 0.0;
        });
        let last = self.data.len() - 1;
        self.data[last] = 1.0;
    }
    pub fn vector_mul(&mut self, vec: &mut [Vec<f32>]) {
        assert!(vec.len() == 2);
        assert_eq!(vec[0].len(), self.width);
        assert_eq!(vec[1].len(), self.height);

        for i in 0..self.height {
            let mut sum = 0.0;
            for j in 0..self.width {
                sum += self.data[j + i * self.width] * vec[1][j];
            }
            vec[1][i] = sum;
        }
    }
}

impl<'a> Layer {
    pub fn print(self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.data[y * self.width + x] > 0.0 {
                    print!("#");
                }
            }
            println!();
        }
    }
}
