use std::cell::LazyCell;
use std::fs::File;
use std::io::BufReader;

use pyo3::prelude::*;
use pyo3::types::PyAny;

mod utilities;
mod circle;
mod square;
mod triangle;
mod factory;

use shapes_lib::circle::Circle;
use shapes_lib::prelude::{Factory, Parser, Shape};

use crate::circle::CircleWrapper;
use crate::square::SquareWrapper;
use crate::triangle::{EquilateralTriangleWrapper, RightTriangleWrapper, TriangleWrapper};
use crate::factory::{ShapeWrapper, ShapeFactory};


#[pymodule]
pub fn shapes_lib_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<CircleWrapper>()?;
    m.add_class::<SquareWrapper>()?;
    m.add_class::<TriangleWrapper>()?;
    m.add_class::<EquilateralTriangleWrapper>()?;
    m.add_class::<RightTriangleWrapper>()?;
    m.add_class::<ShapeWrapper>()?;
    m.add_class::<ShapeFactory>()?;
    Ok(())
}
