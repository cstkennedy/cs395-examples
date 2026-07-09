use std::path::PathBuf;

use itertools::{Itertools};
use ordered_float::OrderedFloat;

use pyo3::prelude::*;

use crate::factory::{ShapeWrapper};
use crate::parser::ShapeParser;

#[pyclass(from_py_object)]
#[derive(Clone, Copy)]
pub enum CompareBy {
    Name,
    Perimeter,
    Area,
}

#[pyclass(skip_from_py_object)]
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
            // TODO: rework &str to String allocation/conversion (DONE)
            CompareBy::Name => self.shapes.sort_by(|lhs, rhs| lhs.name().cmp(rhs.name())),
            CompareBy::Perimeter => self.shapes.sort_by_key(|shp| OrderedFloat(shp.perimeter())),
            CompareBy::Area => self.shapes.sort_by_key(|shp| OrderedFloat(shp.area())),
        };
    }

    pub fn __str__(&self) -> String {
        self.shapes.iter().map(ShapeWrapper::__str__).join("\n\n")
    }
}
