use pyo3::prelude::*;

mod circle;
mod square;
mod triangle;
mod factory;
mod error;

use crate::circle::CircleWrapper;
use crate::square::SquareWrapper;
use crate::triangle::{EquilateralTriangleWrapper, RightTriangleWrapper, TriangleWrapper};
use crate::factory::{ShapeWrapper, ShapeFactory};

#[pymodule]
mod shapes_py {
    use super::*;

    #[pymodule_export]
    use CircleWrapper;

    #[pymodule_export]
    use SquareWrapper;

    #[pymodule_export]
    use TriangleWrapper;

    #[pymodule_export]
    use EquilateralTriangleWrapper;

    #[pymodule_export]
    use RightTriangleWrapper;

    #[pymodule_export]
    use ShapeWrapper;

    #[pymodule_export]
    use ShapeFactory;

    /*
    #[pymodule_init]
    pub fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        pyo3_log::init();

        Ok(())
    }
    */
}
