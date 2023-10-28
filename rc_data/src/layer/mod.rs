use num_traits::{Float, One, Zero};
pub mod activation_fct;
use activation_fct::ActivationFct;

pub struct Layer<'a, T: Float> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
    pub activation_fct: Box<dyn ActivationFct + 'a>,
}

impl<'a, T: Float> Layer<'a, T> {
    pub fn new(width: usize, height: usize, activation_fct: Box<dyn ActivationFct + 'a>) -> Self {
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
}

impl<'a, T: Float> Layer<'a, T> {
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
