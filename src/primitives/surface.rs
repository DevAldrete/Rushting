// Surface Implementation

use super::point::Point;
use crate::primitives::line::Line;

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    center: Point,
    radius: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    top_left: Point,
    width: f64,
    height: f64,
}
