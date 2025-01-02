#[macro_use]
extern crate lazy_static;

pub struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D
{
    pub fn new(some_x: f64, some_y: f64) -> Self {
        Point2D {
            x: some_x,
            y: some_y,
        }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        // Unit testing caught the copy-and-paste error
        self.y
    }

    pub fn get_dims(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}

impl From<(f64, f64)> for Point2D {
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
    fn test_from_tuple_f64() {
        let point = Point2D::from((4.528_f64, 17.2559_f64));

        let expected_dims = (4.528_f64, 17.2559_f64);
        assert_that!(point.get_x(), close_to(expected_dims.0, TOL_F64));
        assert_that!(point.get_y(), close_to(expected_dims.1, TOL_F64));

        let dims = point.get_dims();
        assert_that!(dims.0, close_to(expected_dims.0, TOL_F64));
        assert_that!(dims.1, close_to(expected_dims.1, TOL_F64));
    }
}
