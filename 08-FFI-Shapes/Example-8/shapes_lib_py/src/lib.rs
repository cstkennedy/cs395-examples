use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

use itertools::{Either, Itertools};
use ordered_float::OrderedFloat;

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

#[pyclass]
#[derive(Clone, Copy)]
pub enum CompareBy {
    Name,
    Perimeter,
    Area,
}

#[pyclass]
pub struct ShapeCollection {
    shapes: Vec<ShapeWrapper>,
}

#[pymethods]
impl ShapeCollection {
    #[staticmethod]
    pub fn read_from_file(filename: PathBuf) -> PyResult<Self> {
        Ok(Self {
            shapes: ShapeParser::read_shapes(filename)?,
        })
    }

    /// Get a copy of the "smallest" element using a specified attribute.
    ///
    /// # Raises
    ///
    /// ValueError is ShapeCollection is empty
    ///
    pub fn min(&self, attribute: CompareBy) -> PyResult<ShapeWrapper> {
        if self.shapes.len() == 0 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "ShapeCollection is empty",
            ));
        }

        let smallest = match attribute {
            CompareBy::Name => self.shapes.iter().min_by_key(|shp| shp.name()),
            CompareBy::Perimeter => self
                .shapes
                .iter()
                .min_by_key(|shp| OrderedFloat(shp.perimeter())),
            CompareBy::Area => self
                .shapes
                .iter()
                .min_by_key(|shp| OrderedFloat(shp.area())),
        };

        Ok(smallest
            .ok_or(pyo3::exceptions::PyValueError::new_err(
                "Min could not be found",
            ))?
            .clone())
    }

    /// Get a copy of the "largest" element using a specified attribute.
    ///
    /// # Raises
    ///
    /// ValueError is ShapeCollection is empty
    ///
    pub fn max(&self, attribute: CompareBy) -> PyResult<ShapeWrapper> {
        if self.shapes.len() == 0 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "ShapeCollection is empty",
            ));
        }

        let smallest = match attribute {
            CompareBy::Name => self.shapes.iter().max_by_key(|shp| shp.name()),
            CompareBy::Perimeter => self
                .shapes
                .iter()
                .max_by_key(|shp| OrderedFloat(shp.perimeter())),
            CompareBy::Area => self
                .shapes
                .iter()
                .max_by_key(|shp| OrderedFloat(shp.area())),
        };

        Ok(smallest
            .ok_or(pyo3::exceptions::PyValueError::new_err(
                "Max could not be found",
            ))?
            .clone())
    }

    /// Sort the ShapeCollection in place
    pub fn sort(&mut self, attribute: CompareBy) {
        match attribute {
            // TODO: rework &str to String allocation/conversion
            CompareBy::Name => self.shapes.sort_by_key(|shp| shp.name().to_owned()),
            CompareBy::Perimeter => self.shapes.sort_by_key(|shp| OrderedFloat(shp.perimeter())),
            CompareBy::Area => self.shapes.sort_by_key(|shp| OrderedFloat(shp.area())),
        };
    }

    pub fn __str__(&self) -> String {
        self.shapes.iter().map(ShapeWrapper::__str__).join("\n\n")
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
    m.add_class::<CompareBy>()?;
    m.add_class::<ShapeCollection>()?;
    Ok(())
}
