use pyo3::prelude::*;
use pyo3::types::PyAny;

use shapes::factory::{CreationFactory, FactoryDirectory};
use shapes::monoshape::{MonoFactory, MonoShape};
use shapes::prelude::Shape;
use shapes::{
    circle::Circle, equilateral_triangle::EquilateralTriangle, right_triangle::RightTriangle,
    square::Square, triangle::Triangle,
};

use crate::circle::CircleWrapper;
use crate::error::{ShapeConversionError, ShapeCreationError};
use crate::square::SquareWrapper;
use crate::triangle::{EquilateralTriangleWrapper, RightTriangleWrapper, TriangleWrapper};

#[pyclass(name = "Shape")]
#[derive(Clone)]
pub struct ShapeWrapper {
    wrapped: MonoShape,
}

impl From<MonoShape> for ShapeWrapper {
    fn from(monomorphized: MonoShape) -> Self {
        Self {
            wrapped: monomorphized,
        }
    }
}

impl TryFrom<ShapeWrapper> for Circle {
    type Error = ShapeConversionError;

    fn try_from(shape_wrapper: ShapeWrapper) -> Result<Circle, Self::Error> {
        match shape_wrapper.wrapped {
            MonoShape::Circle { inner } => Ok(inner),
            _ => Err(ShapeConversionError::TypeMismatchError {
                requested: "Circle",
                actual: shape_wrapper.wrapped.name(),
            }),
        }
    }
}

impl TryFrom<ShapeWrapper> for Square {
    type Error = ShapeConversionError;

    fn try_from(shape_wrapper: ShapeWrapper) -> Result<Square, Self::Error> {
        match shape_wrapper.wrapped {
            MonoShape::Square { inner } => Ok(inner),
            _ => Err(ShapeConversionError::TypeMismatchError {
                requested: "Square",
                actual: shape_wrapper.wrapped.name(),
            }),
        }
    }
}

impl TryFrom<ShapeWrapper> for Triangle {
    type Error = ShapeConversionError;

    fn try_from(shape_wrapper: ShapeWrapper) -> Result<Triangle, Self::Error> {
        match shape_wrapper.wrapped {
            MonoShape::Triangle { inner } => Ok(inner),
            _ => Err(ShapeConversionError::TypeMismatchError {
                requested: "Triangle",
                actual: shape_wrapper.wrapped.name(),
            }),
        }
    }
}

impl TryFrom<ShapeWrapper> for EquilateralTriangle {
    type Error = ShapeConversionError;

    fn try_from(shape_wrapper: ShapeWrapper) -> Result<EquilateralTriangle, Self::Error> {
        match shape_wrapper.wrapped {
            MonoShape::EquilateralTriangle { inner } => Ok(inner),
            _ => Err(ShapeConversionError::TypeMismatchError {
                requested: "EquilateralTriangle",
                actual: shape_wrapper.wrapped.name(),
            }),
        }
    }
}

impl TryFrom<ShapeWrapper> for RightTriangle {
    type Error = ShapeConversionError;

    fn try_from(shape_wrapper: ShapeWrapper) -> Result<RightTriangle, Self::Error> {
        match shape_wrapper.wrapped {
            MonoShape::RightTriangle { inner } => Ok(inner),
            _ => Err(ShapeConversionError::TypeMismatchError {
                requested: "RightTriangle",
                actual: shape_wrapper.wrapped.name(),
            }),
        }
    }
}

#[pymethods]
impl ShapeWrapper {
    #[getter]
    pub fn name(&self) -> &str {
        self.wrapped.name()
    }

    pub fn perimeter(&self) -> f64 {
        self.wrapped.perimeter()
    }

    pub fn area(&self) -> f64 {
        self.wrapped.area()
    }

    pub fn __str__(&self) -> String {
        self.wrapped.to_string()
    }

    #[deprecated]
    pub fn trigger_conversion_error(&self) -> Result<(), ShapeConversionError> {
        Err(ShapeConversionError::TypeMismatchError {
            requested: "Circle",
            actual: "Updawg",
        }
        .into())
    }

    pub fn try_as_circle(&self) -> Result<CircleWrapper, ShapeConversionError> {
        let inner_shape: Circle = self.clone().try_into()?;

        Ok(inner_shape.into())
    }

    pub fn try_as_square(&self) -> Result<SquareWrapper, ShapeConversionError> {
        let inner_shape: Square = self.clone().try_into()?;

        Ok(inner_shape.into())
    }

    pub fn try_as_triangle(&self) -> Result<TriangleWrapper, ShapeConversionError> {
        let inner_shape: Triangle = self.clone().try_into()?;

        Ok(inner_shape.into())
    }

    pub fn try_as_equilateral_triangle(
        &self,
    ) -> Result<EquilateralTriangleWrapper, ShapeConversionError> {
        let inner_shape: EquilateralTriangle = self.clone().try_into()?;

        Ok(inner_shape.into())
    }

    pub fn try_as_right_triangle(&self) -> Result<RightTriangleWrapper, ShapeConversionError> {
        let inner_shape: RightTriangle = self.clone().try_into()?;

        Ok(inner_shape.into())
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
        let mono_shape = MonoFactory::create_default(name)?;
        let wrapped_shape: ShapeWrapper = mono_shape.into();

        Ok(wrapped_shape)
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
        let mono_shape = MonoFactory::create_with(name, &dims[..])?;
        let wrapped_shape: ShapeWrapper = mono_shape.into();

        Ok(wrapped_shape)
    }

    /// Determine whether a given shape is known
    ///
    /// # Arguments
    ///
    ///  * `name` the shape for which to query
    ///
    #[staticmethod]
    pub fn is_known(name: &str) -> bool {
        MonoFactory::is_known(&name)
    }

    #[staticmethod]
    pub fn number_known() -> usize {
        MonoFactory::number_known()
    }

    /// List the known shapes, one per line
    ///
    #[staticmethod]
    pub fn list_known() -> String {
        MonoFactory::list_known()
    }
}
