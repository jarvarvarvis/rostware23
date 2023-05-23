pub use crate::xml::common::*;

pub const BOARD_WIDTH: u64 = 8;
pub const BOARD_HEIGHT: u64 = 8;

pub const RIGHTMOST_X: u64 = BOARD_WIDTH * 2 - 1;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector(i64, i64);

impl Vector {
    #[inline] pub fn new(x: i64, y: i64) -> Self {
        Self(x, y)
    }

    #[inline] pub fn between_coordinates(first: Coordinate, second: Coordinate) -> Self {
        Self(first.x() as i64 - second.x() as i64, first.y() as i64 - second.y() as i64)
    }

    #[inline] pub fn scale(&self, scalar: i64) -> Self {
        Self(self.x() * scalar, self.y() * scalar)
    }

    #[inline] pub fn scalar_product(&self, other: Vector) -> i64 {
        self.x() * other.x() + self.y() * other.y() 
    }

    #[inline] pub fn abs(&self) -> f64 {
        let product = self.scalar_product(self.clone());
        (product as f64).sqrt()
    }

    #[inline] pub fn angle_to(&self, other: Vector) -> f64 {
        let v = self.scalar_product(other.clone()) as f64 / (self.abs() * other.abs());
        v.acos()
    }

    #[inline] pub fn x(&self) -> i64 {
        self.0
    }

    #[inline] pub fn y(&self) -> i64 {
        self.1
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Coordinate(u64, u64);

impl Coordinate {
    #[inline] pub fn new(x: u64, y: u64) -> Self {
        Self(x, y)
    }

    #[inline] pub fn x(&self) -> u64 {
        self.0
    }

    #[inline] pub fn y(&self) -> u64 {
        self.1
    }

    #[inline] pub fn add(&self, vector: Vector) -> Self {
        Self::new((self.x() as i64 + vector.x()) as u64, 
                  (self.y() as i64 + vector.y()) as u64)
    }

    #[inline] pub fn odd_r_to_doubled(self) -> Self {
        let x = self.x();
        let y = self.y();
        Self(x * 2 + y % 2, y)
    }

    #[inline] pub fn doubled_to_odd_r(self) -> Self {
        let x = self.x();
        let y = self.y();
        Self((x as f64 / 2.0).ceil() as u64 - y % 2, y)
    }

    #[inline] pub fn is_valid(&self) -> bool {
        let x = self.x();
        let y = self.y();
        x <= RIGHTMOST_X && y < BOARD_HEIGHT && x % 2 == y % 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_r_to_doubled_to_odd_r_results_in_previous_value() {
        let coord = Coordinate::new(6, 2);
        let actual = coord.clone().odd_r_to_doubled().doubled_to_odd_r();
        assert_eq!(coord, actual);
    }
    
    #[test]
    fn doubled_to_odd_r_to_doubled_results_in_previous_value() {
        let coord = Coordinate::new(6, 2);
        let actual = coord.clone().doubled_to_odd_r().odd_r_to_doubled();
        assert_eq!(coord, actual);
    }

    #[test]
    fn take_abs_of_vector() {
        let vector = Vector::new(7, 2);
        let absolute = vector.abs();
        assert_eq!(7.280109889280518, absolute);
    }
    
    #[test]
    fn take_abs_of_vector_with_negative_components() {
        let vector = Vector::new(7, -2);
        let absolute = vector.abs();
        assert_eq!(7.280109889280518, absolute);
    }

    #[test]
    fn vector_between_coordinates_is_correct() {
        let first = Coordinate::new(4, 5);
        let second = Coordinate::new(1, 12);
        let expected = Vector::new(3, -7);
        let actual = Vector::between_coordinates(first, second);
        assert_eq!(expected, actual);
    }

    #[test]
    fn angle_between_same_vectors() {
        let vector = Vector::new(4, 5);
        let actual = vector.angle_to(vector.clone());
        assert_eq!(0.0, actual);
    }

    #[test]
    fn angle_between_vectors_test() {
        let first = Vector::new(3, 5);
        let second = Vector::new(-5, 3);
        let actual = first.angle_to(second);
        assert_eq!(std::f64::consts::PI / 2.0, actual);
    }
}
