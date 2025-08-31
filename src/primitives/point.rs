use super::super::utilities::ApproxEq;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn origin() -> Self {
        Point { x: 0.0, y: 0.0 }
    }

    pub fn to_tuple(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn distance_to(&self, other: Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point: ({},{})", self.x, self.y)
    }
}

impl From<Point> for (f64, f64) {
    fn from(point: Point) -> Self {
        (point.x, point.y)
    }
}

impl From<(f64, f64)> for Point {
    fn from(tuple: (f64, f64)) -> Self {
        Point {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl ApproxEq for Point {
    fn approx_eq(&self, other: &Self, epsilon: f64) -> bool {
        self.x.approx_eq(&other.x, epsilon) && self.y.approx_eq(&other.y, epsilon)
    }
}
