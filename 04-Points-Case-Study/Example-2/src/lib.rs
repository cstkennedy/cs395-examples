#[macro_use]
extern crate lazy_static;

use std::ops::Add;
use std::ops::Sub;

use num_traits::Float;

pub trait PointOps<T>
where
    T: Float,
{
    fn scale(self: &Self, s: T) -> Self;

    fn dot(self: &Self, rhs: &Self) -> T;
}

struct Point2D<T: Float> {
    x: T,
    y: T,
}

impl<T> Point2D<T>
where
    T: Float,
{
    pub fn new(some_x: T, some_y: T) -> Self {
        Point2D {
            x: some_x,
            y: some_y,
        }
    }

    pub fn get_x(&self) -> T {
        self.x
    }

    pub fn get_y(&self) -> T {
        // Unit testing caught the copy-and-paste error
        self.y
    }

    pub fn get_dims(&self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T> PointOps<T> for Point2D<T>
where
    T: Float,
{
    fn scale(self: &Self, s: T) -> Self {
        Point2D::new(self.x * s, self.y * s)
    }

    fn dot(self: &Self, rhs: &Self) -> T {
        (self.x * rhs.x) + (self.y * rhs.y)
    }
}

impl<T> Sub for Point2D<T>
where
    T: Float + Sub,
{
    type Output = Point2D<T>;

    fn sub(self, rhs: Self) -> Self {
        Point2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Add for Point2D<T>
where
    T: Float + Add,
{
    type Output = Point2D<T>;

    fn add(self, rhs: Self) -> Self {
        // Unit test caught copy and paste error (- vs +)
        Point2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl From<(f32, f32)> for Point2D<f32> {
    fn from(tuple: (f32, f32)) -> Self {
        Point2D::new(tuple.0, tuple.1)
    }
}

impl From<(f64, f64)> for Point2D<f64> {
    fn from(tuple: (f64, f64)) -> Self {
        Point2D::new(tuple.0, tuple.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use hamcrest2::prelude::*;

    // Tolerance within which two numbers are considered equal
    const TOL_F64: f64 = 1e-8;
    const TOL_F32: f32 = 1e-8;

    #[test]
    fn test_origin_f64() {
        let origin = Point2D::new(0_f64, 0_f64);

        assert_that!(origin.get_x(), close_to(0_f64, TOL_F64));
        assert_that!(origin.get_y(), close_to(0_f64, TOL_F64));

        let dims = origin.get_dims();
        assert_that!(dims.0, close_to(0_f64, TOL_F64));
        assert_that!(dims.1, close_to(0_f64, TOL_F64));
    }

    #[test]
    fn test_origin_f32() {
        let origin = Point2D::new(0_f32, 0_f32);

        assert_that!(origin.get_x(), close_to(0_f32, TOL_F32));
        assert_that!(origin.get_y(), close_to(0_f32, TOL_F32));

        let dims = origin.get_dims();
        assert_that!(dims.0, close_to(0_f32, TOL_F32));
        assert_that!(dims.1, close_to(0_f32, TOL_F32));
    }

    #[test]
    fn test_from_tuple_f32() {
        let point: Point2D<f32> = (1.5_f32, 17.2_f32).into();

        let expected_dims = (1.5_f32, 17.2_f32);
        assert_that!(point.get_x(), close_to(expected_dims.0, TOL_F32));
        assert_that!(point.get_y(), close_to(expected_dims.1, TOL_F32));

        let dims = point.get_dims();
        assert_that!(dims.0, close_to(expected_dims.0, TOL_F32));
        assert_that!(dims.1, close_to(expected_dims.1, TOL_F32));
    }

    #[test]
    fn test_from_tuple_f64() {
        let point = Point2D::from((4.528_f64, 17.2559_f64));

        let expected_dims = (4.528_f64, 17.2559_f64);
        assert_that!(point.get_x(), close_to(expected_dims.0, TOL_F64));
        assert_that!(point.get_y(), close_to(expected_dims.1, TOL_F64));

        let dims = point.get_dims();
        assert_that!(dims.0, close_to(expected_dims.0, TOL_F64));
        assert_that!(dims.1, close_to(expected_dims.1, TOL_F64));
    }

    #[test]
    fn test_add() {
        let pt1 = Point2D::from((1.3_f64, 2.9_f64));
        let pt2 = Point2D::from((1.0_f64, 2.5_f64));

        let expected_dims = (2.3_f64, 5.4_f64);

        let dims = (pt1 + pt2).get_dims();
        assert_that!(dims.0, close_to(expected_dims.0, TOL_F64));
        assert_that!(dims.1, close_to(expected_dims.1, TOL_F64));
    }

    #[test]
    fn test_sub() {
        let pt1 = Point2D::from((1.3_f64, 2.9_f64));
        let pt2 = Point2D::from((1.0_f64, 2.5_f64));

        let expected_dims = (0.3_f64, 0.4_f64);

        let dims = (pt1 - pt2).get_dims();
        assert_that!(dims.0, close_to(expected_dims.0, TOL_F64));
        assert_that!(dims.1, close_to(expected_dims.1, TOL_F64));
    }

    #[test]
    fn test_scale() {
        let pt = Point2D::from((1.0_f64, 2.5_f64)).scale(2_f64);

        let expected_dims = (2.0_f64, 5.0_f64);

        let dims = pt.get_dims();
        assert_that!(dims.0, close_to(expected_dims.0, TOL_F64));
        assert_that!(dims.1, close_to(expected_dims.1, TOL_F64));
    }

    #[test]
    fn test_dot_product() {
        let pt1 = Point2D::from((1.3_f64, 2.9_f64));
        let pt2 = Point2D::from((1.0_f64, 2.0_f64));

        let expected = 7.1_f64;

        assert_that!(pt1.dot(&pt2), close_to(expected, TOL_F64));
    }
}
