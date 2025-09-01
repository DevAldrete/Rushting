use std::fmt::Display;
use std::ops::Add;

use super::point::Point;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Self {
        Line { p1, p2 }
    }

    pub fn length(&self) -> f64 {
        self.p1.distance_to(self.p2)
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line from {} to {}", self.p1, self.p2)
    }
}

// impl Add for Line {
//     type Output = Line;
//
//     fn add(self, rhs: Self) -> Self::Output {
//         Line {}
//     }
// }
