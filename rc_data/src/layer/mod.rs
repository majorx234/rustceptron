use num_traits::{Float, One, Zero};
pub mod activation_fct;
use activation_fct::ActivationFct;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Layer<T: Float + std::ops::AddAssign> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
    activation_fct: Box<dyn ActivationFct + 'static>,
}

impl<T: Float + std::ops::AddAssign> Layer<T> {
    pub fn new(width: usize, height: usize, activation_fct: Box<dyn ActivationFct>) -> Self {
        Self {
            width,
            height,
            data: vec![Zero::zero(); width * height + 1],
            activation_fct,
        }
    }
    pub fn clear(&mut self) {
        self.data.iter_mut().for_each(|x| {
            *x = Zero::zero();
        });
        let last = self.data.len() - 1;
        self.data[last] = One::one();
    }
    pub fn vector_mul(&mut self, in_vec: Vec<T>, out_vec: &mut Vec<T>) {
        assert_eq!(in_vec.len(), self.width);
        assert_eq!(out_vec.len(), self.height);

        for i in 0..self.height {
            out_vec[i] = T::zero();
            for j in 0..self.width {
                out_vec[i] += self.data[j + i * self.width];
            }
        }
    }
}

impl<'a, T: Float + std::ops::AddAssign> Layer<T> {
    pub fn print(self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.data[y * self.width + x] > Zero::zero() {
                    print!("#");
                }
            }
            println!();
        }
    }
}
