use pyo3::prelude::*;

mod circle;
mod square;
mod triangle;

use crate::circle::CircleWrapper;
use crate::square::SquareWrapper;
use crate::triangle::{EquilateralTriangleWrapper, RightTriangleWrapper, TriangleWrapper};

// Old - Procedural
/*
#[pymodule]
pub fn shapes_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();

    m.add_class::<CircleWrapper>()?;
    m.add_class::<SquareWrapper>()?;
    m.add_class::<TriangleWrapper>()?;
    m.add_class::<EquilateralTriangleWrapper>()?;
    m.add_class::<RightTriangleWrapper>()?;
    Ok(())
}
*/

// New -  Declarative
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

    /*
    #[pymodule_init]
    pub fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        pyo3_log::init();

        Ok(())
    }
    */
}
