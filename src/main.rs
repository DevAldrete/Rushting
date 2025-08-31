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
    use super::primitives::{line::Line, point::Point};

    #[test]
    fn can_make_line_segment_from_two_points() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(3.0, 4.0);
        let segment = Line::new(a, b);
        assert_eq!(segment.p1, a);
        assert_eq!(segment.p2, b);
    }
}
