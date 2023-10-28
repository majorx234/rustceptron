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
