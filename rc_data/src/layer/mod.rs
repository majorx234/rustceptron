use num_traits::{Float, One, Zero};
mod activation_fct;

pub struct Layer<T: Float> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl<T: Float> Layer<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![Zero::zero(); width * height + 1],
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

impl<T: Float> Layer<T> {
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
