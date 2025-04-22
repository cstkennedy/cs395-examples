use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

use itertools::{Either, Itertools};

use log;

use pyo3::prelude::*;

mod circle;
mod error;
mod factory;
mod square;
mod triangle;
mod utilities;

use crate::circle::CircleWrapper;
use crate::error::ShapeCreationError;
use crate::factory::{ShapeFactory, ShapeWrapper};
use crate::square::SquareWrapper;
use crate::triangle::{EquilateralTriangleWrapper, RightTriangleWrapper, TriangleWrapper};

#[pyclass]
pub struct ShapeParser;

#[pymethods]
impl ShapeParser {
    #[staticmethod]
    pub fn read_shape(line: &str) -> Result<ShapeWrapper, ShapeCreationError> {
        // Ok(Circle::new(1.0).into())
        let split_line: Vec<&str> = line.trim().split(";").collect();

        if split_line.len() != 2 {
            return Err(ShapeCreationError::MalformedLineError(format!(
                "Line '{line}' did not have exactly one (1) semicolon"
            )));
        }

        let name = split_line[0];
        let values: Vec<f64> = split_line[1]
            .split_whitespace()
            .flat_map(|token| token.parse())
            .collect();

        let shape = ShapeFactory::create_with(name, values)?;

        Ok(shape)
    }

    #[staticmethod]
    fn read_shapes(file_path: PathBuf) -> PyResult<Vec<ShapeWrapper>> {
        let shape_file = File::open(&file_path).map_err(|_| {
            pyo3::exceptions::PyFileNotFoundError::new_err(format!("{file_path:?} does not exist"))
        })?;

        let buffer = BufReader::new(shape_file);

        let (shapes, errors): (Vec<_>, Vec<_>) = buffer
            .lines()
            .flatten()
            .map(|line| ShapeParser::read_shape(&line))
            .partition_map(|result| match result {
                Ok(shape) => Either::Left(shape),
                Err(err) => Either::Right(err),
            });

        for err in errors {
            log::warn!("{}", err);
        }

        Ok(shapes)
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
