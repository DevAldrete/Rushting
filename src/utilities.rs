pub trait ApproxEq {
    fn approx_eq(&self, other: &Self, epsilon: f64) -> bool;
}

impl ApproxEq for f64 {
    fn approx_eq(&self, other: &Self, epsilon: f64) -> bool {
        (self - other).abs() < epsilon
    }
}
