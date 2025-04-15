use pyo3::prelude::*;

mod circle;
mod square;
mod triangle;

use crate::circle::CircleWrapper;
use crate::square::SquareWrapper;
use crate::triangle::{EquilateralTriangleWrapper, RightTriangleWrapper, TriangleWrapper};

#[pymodule]
pub fn shapes_lib_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<CircleWrapper>()?;
    m.add_class::<SquareWrapper>()?;
    m.add_class::<TriangleWrapper>()?;
    m.add_class::<EquilateralTriangleWrapper>()?;
    m.add_class::<RightTriangleWrapper>()?;
    Ok(())
}
