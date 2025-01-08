use std::ops::Add;
use std::ops::Sub;

use super::traits::PointOps;
use num_traits::Float;

/// A Point in 2-D Cartesian Coordinates.
#[derive(Debug, Clone, Copy)]
pub struct Point2D<T: Float> {
    x: T,
    y: T,
}

impl<T> Point2D<T>
where
    T: Float,
{
    /// Creates a new point for a given x and y.
    ///
    /// # Arguments
    ///
    ///   * `some_x` - desired x value
    ///   * `some_y` - desired y value
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_points::point2d::Point2D;
    ///
    /// let origin = Point2D::new(0_f64, 0_f64);
    /// let some_point = Point2D::new(0.5_f64, 9.2_f64);
    /// ```
    pub fn new(some_x: T, some_y: T) -> Self {
        Point2D {
            x: some_x,
            y: some_y,
        }
    }

    /// Returns x component.
    pub fn get_x(&self) -> T {
        self.x
    }

    /// Returns y component.
    pub fn get_y(&self) -> T {
        // Unit testing caught the copy-and-paste error
        self.y
    }

    /// Returns x and y as a tuple in the form (x, y).
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
