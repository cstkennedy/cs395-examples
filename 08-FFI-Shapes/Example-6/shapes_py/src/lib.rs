use pyo3::prelude::*;

pub mod circle;
pub mod error;
pub mod factory;
pub mod square;
pub mod triangle;
pub mod parser;

use crate::circle::CircleWrapper;
use crate::factory::{ShapeFactory, ShapeWrapper};
use crate::parser::ShapeParser;
use crate::square::SquareWrapper;
use crate::triangle::{EquilateralTriangleWrapper, RightTriangleWrapper, TriangleWrapper};

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

    #[pymodule_export]
    use ShapeParser;

    #[pymodule_init]
    pub fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        pyo3_log::init();

        Ok(())
    }
}
