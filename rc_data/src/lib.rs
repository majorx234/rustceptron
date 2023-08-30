use num_traits::{Float, Zero};

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
            data: vec![Zero::zero(); width * height],
        }
    }
}

impl<T: Float> Layer<T> {
    pub fn print(self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.data[y * self.width + x] > Zero::zero() {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}
