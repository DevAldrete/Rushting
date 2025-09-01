// Surface Implementation

use super::point::Point;
use crate::{primitives::line::Line, utilities::ApproxEq};

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    center: Point,
    radius: f64,
}

impl Circle {
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn get_center(&self) -> Point {
        self.center
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }

    pub fn contains(&self, p: Point) -> bool {
        self.get_center()
            .distance_to(p)
            .approx_eq(&self.get_radius(), 1e-12)
    }

    pub fn on_circle(&self, p: Point) -> bool {
        self.get_center()
            .distance_to(p)
            .approx_eq(&self.get_radius(), 1e-12)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    top_left: Point,
    width: f64,
    height: f64,
}
