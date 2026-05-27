use pyo3::prelude::*;

use shapes_lib::prelude::Factory;
use shapes_lib::{
    circle::Circle, equilateral_triangle::EquilateralTriangle, right_triangle::RightTriangle,
    square::Square, triangle::Triangle,
};

use crate::circle::CircleWrapper;
use crate::error::ShapeCreationError;
use crate::square::SquareWrapper;
use crate::triangle::{EquilateralTriangleWrapper, RightTriangleWrapper, TriangleWrapper};

#[pyclass(name = "Shape")]
#[derive(Clone)]
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

impl From<Square> for ShapeWrapper {
    fn from(val: Square) -> Self {
        ShapeWrapper::Square(val.into())
    }
}

impl From<Triangle> for ShapeWrapper {
    fn from(val: Triangle) -> Self {
        ShapeWrapper::Triangle(val.into())
    }
}

impl From<EquilateralTriangle> for ShapeWrapper {
    fn from(val: EquilateralTriangle) -> Self {
        ShapeWrapper::EquilateralTriangle(val.into())
    }
}

impl From<RightTriangle> for ShapeWrapper {
    fn from(val: RightTriangle) -> Self {
        ShapeWrapper::RightTriangle(val.into())
    }
}

impl From<shapes_lib::prelude::MonoShape> for ShapeWrapper {
    fn from(monomorphized: shapes_lib::prelude::MonoShape) -> Self {
        match monomorphized {
            shapes_lib::prelude::MonoShape::Circle { inner } => inner.into(),
            shapes_lib::prelude::MonoShape::Square { inner } => inner.into(),
            shapes_lib::prelude::MonoShape::Triangle { inner } => inner.into(),
            shapes_lib::prelude::MonoShape::EquilateralTriangle { inner } => inner.into(),
            shapes_lib::prelude::MonoShape::RightTriangle { inner } => inner.into(),
        }
    }
}

#[pymethods]
impl ShapeWrapper {
    #[getter]
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
    pub fn create(name: &str) -> Result<ShapeWrapper, ShapeCreationError> {
        Ok(ShapeWrapper::from(shapes_lib::prelude::Factory::create(name)?))
    }

    /// Create a Shape with specified dimensions.
    ///
    /// # Arguments
    ///
    ///   * `name` shape to be created
    ///   * `dims` input dimensions
    ///
    #[staticmethod]
    pub fn create_with(name: &str, dims: Vec<f64>) -> Result<ShapeWrapper, ShapeCreationError> {
        Ok(ShapeWrapper::from(shapes_lib::prelude::Factory::create_with(name, &dims[..])?))
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
