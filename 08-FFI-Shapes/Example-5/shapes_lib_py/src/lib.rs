use std::fs::File;
use std::io::BufReader;

use log;

use pyo3::prelude::*;

mod utilities;
mod circle;
mod square;
mod triangle;
mod factory;

use shapes_lib::circle::Circle;

use crate::circle::CircleWrapper;
use crate::square::SquareWrapper;
use crate::triangle::{EquilateralTriangleWrapper, RightTriangleWrapper, TriangleWrapper};
use crate::factory::{ShapeWrapper, ShapeFactory};

#[pyclass]
pub struct ShapeParser;

#[pymethods]
impl ShapeParser {
    #[staticmethod]
    pub fn read_shape(line: &str) -> PyResult<ShapeWrapper> {
        // Ok(Circle::new(1.0).into())
        let split_line: Vec<&str> = line.trim().split(";").collect();

        if split_line.len() != 2 {
            return Err(pyo3::exceptions::PyValueError::new_err(format!("Line '{line}' did not have exactly one (1) semicolon")));
        }

        let name = split_line[0];
        let values: Vec<f64> = split_line[1]
            .split_whitespace()
            .flat_map(|token| token.parse())
            .collect();

        let shape = ShapeFactory::create_with(name, values)?;

        Ok(shape)
    }
}



#[pymodule]
pub fn shapes_lib_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();

    m.add_class::<CircleWrapper>()?;
    m.add_class::<SquareWrapper>()?;
    m.add_class::<TriangleWrapper>()?;
    m.add_class::<EquilateralTriangleWrapper>()?;
    m.add_class::<RightTriangleWrapper>()?;
    m.add_class::<ShapeWrapper>()?;
    m.add_class::<ShapeFactory>()?;
    m.add_class::<ShapeParser>()?;
    Ok(())
}
