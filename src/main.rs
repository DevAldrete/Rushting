mod primitives;
mod utilities;

fn main() {
    // let p1 = Point::new(3.0);
    // let p2 = Point::new(7.0);
    // let line = Line::new(p1, p2);
    // println!("{}, {}, {}", p1, p2, line);
    // println!("Length of line: {}", line.length());
}

#[cfg(test)]
mod tests {
    use crate::utilities::ApproxEq;

    use super::primitives::{line::Line, point::Point, surfaces::Circle};

    #[test]
    fn can_make_line_segment_from_two_points() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(3.0, 4.0);
        let segment = Line::new(a, b);
        assert_eq!(segment.p1, a);
        assert_eq!(segment.p2, b);
    }

    #[test]
    fn point_on_circle_satisfies_radius() {
        let c = Circle::new(Point::origin(), 5.0);
        let p = Point::new(3.0, 4.0);
        assert!(c.contains(p));
    }

    #[test]
    fn distance_is_zero_for_same_points() {
        let p = Point::new(2.0, 3.0);
        assert!(p.distance_to(p).approx_eq(&0.0, 1e-12));
    }

    #[test]
    fn distance_is_symmetric() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(4.0, 6.0);
        assert!(p1.distance_to(p2).approx_eq(&p2.distance_to(p1), 1e-12));
    }

    #[test]
    fn triangle_inequality_holds() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        let p3 = Point::new(6.0, 8.0);

        let ab = p1.distance_to(p2);
        let bc = p2.distance_to(p3);
        let ac = p1.distance_to(p3);
        assert!(ac <= ab + bc);
    }

    #[test]
    fn line_segment_stores_correctly_points() {
        let a = Point::origin();
        let b = Point::new(3.0, 4.0);
        let segment = Line::new(a, b);
        assert_eq!(segment.p1, a);
        assert_eq!(segment.p2, b);
    }

    #[test]
    fn line_segment_length_is_correct() {
        let a = Point::origin();
        let b = Point::new(3.0, 4.0);
        let segment = Line::new(a, b);
        assert!(segment.length().approx_eq(&a.distance_to(b), 1e-12));
    }

    #[test]
    fn colinearity_of_segment() {
        let a = Point::new(5.0, 5.0);
        let segment = Line::new(a, a);

        assert!(segment.length().approx_eq(&0.0, 1e-12))
    }
}
