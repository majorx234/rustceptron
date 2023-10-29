use std::f32::consts::E;

pub trait ActivationFct {
    fn fvalue(&self, x: f32) -> f32;
    fn derivation(&self, x: f32) -> Option<f32>;
}

pub struct ReLU {
    slope: f32,
}

impl ReLU {
    pub fn new(slope: f32) -> Self {
        ReLU { slope }
    }
}

impl ActivationFct for ReLU {
    fn fvalue(&self, x: f32) -> f32 {
        if x < 0.0 {
            return 0.0;
        }
        x * self.slope
    }
    fn derivation(&self, x: f32) -> Option<f32> {
        if x < 0.0 {
            return Some(0.0);
        }
        Some(self.slope)
    }
}

pub struct Sigmoid {}

impl Sigmoid {
    fn new() -> Sigmoid {
        Sigmoid {}
    }
}

impl ActivationFct for Sigmoid {
    fn fvalue(&self, x: f32) -> f32 {
        1.0 / (1.0 + E.powf(-x))
    }
    fn derivation(&self, x: f32) -> Option<f32> {
        Some(E.powf(x) / (1.0 + E.powf(x)).powf(2.0))
    }
}
