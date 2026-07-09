use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;

use log;

use pyo3::prelude::*;

use shapes::monoshape::MonoShape;

use crate::error::ShapeCreationError;
use crate::factory::{ShapeFactory, ShapeWrapper};


impl FromStr for ShapeWrapper {
    type Err = ShapeCreationError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let monoshape = line.parse::<MonoShape>()?;
        let wrapped = ShapeWrapper::from(monoshape);

        Ok(wrapped)
    }
}


#[pyclass]
pub struct ShapeParser;

#[pymethods]
impl ShapeParser {
    #[staticmethod]
    pub fn read_shape(line: &str) -> Result<ShapeWrapper, ShapeCreationError> {
        line.parse()
    }

    #[staticmethod]
    pub fn read_shapes(file_path: PathBuf) -> PyResult<Vec<ShapeWrapper>> {
        let shape_file = File::open(&file_path).map_err(|_| {
            pyo3::exceptions::PyFileNotFoundError::new_err(format!("{file_path:?} does not exist"))
        })?;

        let buffer = BufReader::new(shape_file);

        let shapes: Vec<_> = buffer
            .lines()
            .flatten()
            .map(|line| ShapeParser::read_shape(&line))
            .inspect(|result| {
                if let Err(err) = result {
                    log::warn!("{}", err);
                }
            })
            .flatten()
            .collect();

        Ok(shapes)
    }
}

