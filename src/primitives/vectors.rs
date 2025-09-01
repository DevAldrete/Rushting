// Vectors (no Eucladian part)

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector2<T> {
    x: T,
    y: T,
}

trait VectorOps<T> {
    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
    fn scale(self, scalar: T) -> Self;
    fn dot(self, other: Self) -> T;
    fn cross(self, other: Self) -> T
    where
        T: std::ops::Mul<Output = T> + std::ops::Sub<Output = T> + Copy;
}

impl VectorOps<f64> for Vector2<f64> {
    fn add(self, other: Self) -> Self {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(self, other: Self) -> Self {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn scale(self, scalar: f64) -> Self {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    fn cross(self, other: Self) -> f64
    where
        f64: std::ops::Mul<Output = f64> + std::ops::Sub<Output = f64> + Copy,
    {
    }
}
