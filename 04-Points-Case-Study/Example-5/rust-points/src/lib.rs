#[macro_use]
extern crate lazy_static;

pub mod point2d;
pub mod traits;

pub mod prelude {
    pub use crate::point2d::Point2D;
    pub use crate::traits::PointOps;
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn test_prelude() {
        let point = Point2D::new(1_f64, 9_f64);
    }
}
