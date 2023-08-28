use num_traits::{Float, Zero};

pub struct Layer<T: Float> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl<T: num_traits::Float> Layer<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![Zero::zero(); width * height],
        }
    }
}
