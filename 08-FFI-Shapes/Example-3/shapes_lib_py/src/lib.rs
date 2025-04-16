use std::cell::LazyCell;
use std::fs::File;
use std::io::BufReader;

use pyo3::prelude::*;
use pyo3::types::PyAny;

mod circle;
mod square;
mod triangle;

use shapes_lib::circle::Circle;
use shapes_lib::prelude::{Factory, Parser, Shape};

use crate::circle::CircleWrapper;
use crate::square::SquareWrapper;
use crate::triangle::{EquilateralTriangleWrapper, RightTriangleWrapper, TriangleWrapper};

#[pyclass(name = "Shape")]
pub enum ShapeWrapper {
    Circle(CircleWrapper),
    Square(SquareWrapper),
    Triangle(TriangleWrapper),
    EquilateralTriangle(EquilateralTriangleWrapper),
    RightTriangle(RightTriangleWrapper),
}

impl From<Circle> for ShapeWrapper {
    fn from(val: Circle) -> Self {
        ShapeWrapper::Circle(val.into())
    }
}

#[pymethods]
impl ShapeWrapper {
    pub fn name(&self) -> &str {
        match &self {
            ShapeWrapper::Circle(wrapped) => wrapped.name(),
            ShapeWrapper::Square(wrapped) => wrapped.name(),
            ShapeWrapper::Triangle(wrapped) => wrapped.name(),
            ShapeWrapper::EquilateralTriangle(wrapped) => wrapped.name(),
            ShapeWrapper::RightTriangle(wrapped) => wrapped.name(),
        }
    }

    pub fn perimeter(&self) -> f64 {
        match &self {
            ShapeWrapper::Circle(wrapped) => wrapped.perimeter(),
            ShapeWrapper::Square(wrapped) => wrapped.perimeter(),
            ShapeWrapper::Triangle(wrapped) => wrapped.perimeter(),
            ShapeWrapper::EquilateralTriangle(wrapped) => wrapped.perimeter(),
            ShapeWrapper::RightTriangle(wrapped) => wrapped.perimeter(),
        }
    }

    pub fn area(&self) -> f64 {
        match &self {
            ShapeWrapper::Circle(wrapped) => wrapped.area(),
            ShapeWrapper::Square(wrapped) => wrapped.area(),
            ShapeWrapper::Triangle(wrapped) => wrapped.area(),
            ShapeWrapper::EquilateralTriangle(wrapped) => wrapped.area(),
            ShapeWrapper::RightTriangle(wrapped) => wrapped.area(),
        }
    }

    pub fn __str__(&self) -> String {
        match &self {
            ShapeWrapper::Circle(wrapped) => wrapped.__str__(),
            ShapeWrapper::Square(wrapped) => wrapped.__str__(),
            ShapeWrapper::Triangle(wrapped) => wrapped.__str__(),
            ShapeWrapper::EquilateralTriangle(wrapped) => wrapped.__str__(),
            ShapeWrapper::RightTriangle(wrapped) => wrapped.__str__(),
        }
    }
}

#[pyclass]
pub struct ShapeFactory;

#[pymethods]
impl ShapeFactory {
    /// Create a Shape
    ///
    /// # Arguments
    ///
    ///   * `name` shape to be created
    ///
    #[staticmethod]
    pub fn create(name: &str) -> PyResult<ShapeWrapper> {
        match name {
            "Circle" => Ok(Circle::default().into()),
            _ => Err(pyo3::exceptions::PyKeyError::new_err(format!(
                "'{name}' is not known"
            ))),
        }
    }

    /// Create a Shape with specified dimensions.
    ///
    /// # Arguments
    ///
    ///   * `name` shape to be created
    ///   * `dims` input dimensions
    ///
    #[staticmethod]
    pub fn create_with(name: &str, dims: Vec<f64>) -> PyResult<ShapeWrapper> {
        match name {
            "Circle" => Ok(Circle::from(&dims[..]).into()),
            _ => Err(pyo3::exceptions::PyKeyError::new_err(format!(
                "'{name}' is not known"
            ))),
        }
    }

    /// Determine whether a given shape is known
    ///
    /// # Arguments
    ///
    ///  * `name` the shape for which to query
    ///
    #[staticmethod]
    pub fn is_known(name: &str) -> bool {
        Factory::is_known(&name)
    }

    #[staticmethod]
    pub fn number_known() -> usize {
        Factory::number_known()
    }

    /// List the known shapes, one per line
    ///
    #[staticmethod]
    pub fn list_known() -> String {
        Factory::list_known()
    }
}

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
