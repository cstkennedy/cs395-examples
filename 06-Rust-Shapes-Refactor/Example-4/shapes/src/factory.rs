use crate::circle::Circle;
use crate::equilateral_triangle::EquilateralTriangle;
use crate::error::CreationError;
use crate::right_triangle::RightTriangle;
use crate::shape::Shape;
use crate::square::Square;
use crate::triangle::Triangle;

use std::cell::LazyCell;

use itertools::Itertools;

impl From<Circle> for Box<dyn Shape> {
    fn from(shape: Circle) -> Self {
        Box::new(shape)
    }
}

impl From<Square> for Box<dyn Shape> {
    fn from(shape: Square) -> Self {
        Box::new(shape)
    }
}

impl From<Triangle> for Box<dyn Shape> {
    fn from(shape: Triangle) -> Self {
        Box::new(shape)
    }
}

impl From<EquilateralTriangle> for Box<dyn Shape> {
    fn from(shape: EquilateralTriangle) -> Self {
        Box::new(shape)
    }
}

impl From<RightTriangle> for Box<dyn Shape> {
    fn from(shape: RightTriangle) -> Self {
        Box::new(shape)
    }
}

impl TryFrom<&[f64]> for Triangle {
    type Error = CreationError;

    fn try_from(dims: &[f64]) -> Result<Self, Self::Error> {
        if dims.len() != 3 {
            return Err(CreationError::DimensionCountError {
                name: "Triangle".to_owned(),
                num_required: 3,
            });
        }
        Ok(Triangle {
            side_a: dims[0],
            side_b: dims[1],
            side_c: dims[2],
        })
    }
}

impl TryFrom<&[f64]> for EquilateralTriangle {
    type Error = CreationError;

    fn try_from(dims: &[f64]) -> Result<Self, Self::Error> {
        if dims.len() != 1 {
            return Err(CreationError::DimensionCountError {
                name: "Equilateral Triangle".to_owned(),
                num_required: 1,
            });
        }
        Ok(EquilateralTriangle { side: dims[0] })
    }
}

impl TryFrom<&[f64]> for RightTriangle {
    type Error = CreationError;

    fn try_from(dims: &[f64]) -> Result<Self, Self::Error> {
        if dims.len() != 2 {
            return Err(CreationError::DimensionCountError {
                name: "Right Triangle".to_owned(),
                num_required: 2,
            });
        }
        Ok(RightTriangle {
            base: dims[0],
            height: dims[1],
        })
    }
}

impl TryFrom<&[f64]> for Circle {
    type Error = CreationError;

    fn try_from(dims: &[f64]) -> Result<Self, Self::Error> {
        if dims.len() != 1 {
            return Err(CreationError::DimensionCountError {
                name: "Circle".to_owned(),
                num_required: 1,
            });
        }
        Ok(Circle { radius: dims[0] })
    }
}

impl TryFrom<&[f64]> for Square {
    type Error = CreationError;

    fn try_from(dims: &[f64]) -> Result<Self, Self::Error> {
        if dims.len() != 1 {
            return Err(CreationError::DimensionCountError {
                name: "Square".to_owned(),
                num_required: 1,
            });
        }

        Ok(Square { side: dims[0] })
    }
}

//------------------------------------------------------------------------------
// Factory Traits
//------------------------------------------------------------------------------
pub trait CreationFactory {
    type Item;
    type Error;

    fn create_default(name: &str) -> Result<Self::Item, Self::Error>;

    fn create_with(name: &str, dims: &[f64]) -> Result<Self::Item, Self::Error>;
}

pub trait FactoryDirectory {
    fn is_known(name: &str) -> bool;

    fn number_known() -> usize;

    fn list_known() -> String;
}

//------------------------------------------------------------------------------
// Factory Implementation
//------------------------------------------------------------------------------
type DefaultFunction = dyn Fn() -> Box<dyn Shape>;
type DimFunction = dyn Fn(&[f64]) -> Result<Box<dyn Shape>, CreationError>;
type ShapeTuple<'a> = (&'a str, Box<DefaultFunction>, Box<DimFunction>);

#[rustfmt::skip]
const CREATE_SHAPE: LazyCell<Vec<ShapeTuple>> = LazyCell::new(|| {
    vec![
        (
            "Triangle",
            Box::new(|| Triangle::default().into()),
            Box::new(|dims| Ok(Triangle::try_from(dims)?.into())),
        ),
        (
            "Right Triangle",
            Box::new(|| RightTriangle::default().into()),
            Box::new(|dims| Ok(RightTriangle::try_from(dims)?.into())),
        ),
        (
            "Equilateral Triangle",
            Box::new(|| EquilateralTriangle::default().into()),
            Box::new(|dims| Ok(EquilateralTriangle::try_from(dims)?.into())),
        ),
        (
            "Square",
            Box::new(|| Square::default().into()),
            Box::new(|dims| Ok(Square::try_from(dims)?.into())),
        ),
        (
            "Circle",
            Box::new(|| Circle::default().into()),
            Box::new(|dims| Ok(Circle::try_from(dims)?.into())),
        ),
    ]
});

pub struct Factory;

impl CreationFactory for Factory {
    type Item = Box<dyn Shape>;
    type Error = CreationError;

    /// Create a Shape
    ///
    /// # Arguments
    ///
    ///   * `name` shape to be created
    ///
    fn create_default(name: &str) -> Result<Self::Item, Self::Error> {
        match CREATE_SHAPE
            .iter()
            .find(|(shape_name, _, _)| shape_name == &name)
        {
            Some((_, creation_op, _)) => Ok(creation_op().into()),
            None => Err(CreationError::UnknownShapeError(name.to_owned())),
        }
    }

    /// Create a Shape with specified dimensions.
    ///
    /// # Arguments
    ///
    ///   * `name` shape to be created
    ///   * `dims` input dimensions
    ///
    fn create_with(name: &str, dims: &[f64]) -> Result<Self::Item, Self::Error> {
        match CREATE_SHAPE
            .iter()
            .find(|(shape_name, _, _)| shape_name == &name)
        {
            None => Err(CreationError::UnknownShapeError(name.to_owned())),
            Some((_, _, creation_op)) => creation_op(&dims),
        }
    }
}

impl FactoryDirectory for Factory {
    /// Determine whether a given shape is known
    ///
    /// # Arguments
    ///
    ///  * `name` the shape for which to query
    ///
    fn is_known(name: &str) -> bool {
        CREATE_SHAPE
            .iter()
            .find(|(shape_name, _, _)| shape_name == &name)
            .is_some()
    }

    fn number_known() -> usize {
        CREATE_SHAPE.len()
    }

    /// List the known shapes, one per line
    ///
    fn list_known() -> String {
        CREATE_SHAPE
            .iter()
            .map(|(name, _, _)| format!("  {}", name))
            .join("\n")
            + "\n"
    }
}

//------------------------------------------------------------------------------
// Mono Shape Implementation
//------------------------------------------------------------------------------
/// A Monomorphized Wrapper for all Known Shapes.
///
/// Note: This is the first step in reducing duplicated FFI factory logic.
#[derive(Clone, Debug)]
pub enum MonoShape {
    Circle { inner: Circle },
    Square { inner: Square },
    Triangle { inner: Triangle },
    EquilateralTriangle { inner: EquilateralTriangle },
    RightTriangle { inner: RightTriangle },
}

// TODO: replace with macro
impl Shape for MonoShape {
    fn name(&self) -> &'static str {
        match *&self {
            MonoShape::Circle { inner } => inner.name(),
            MonoShape::Square { inner } => inner.name(),
            MonoShape::Triangle { inner } => inner.name(),
            MonoShape::EquilateralTriangle { inner } => inner.name(),
            MonoShape::RightTriangle { inner } => inner.name(),
        }
    }

    fn area(&self) -> f64 {
        match *&self {
            MonoShape::Circle { inner } => inner.area(),
            MonoShape::Square { inner } => inner.area(),
            MonoShape::Triangle { inner } => inner.area(),
            MonoShape::EquilateralTriangle { inner } => inner.area(),
            MonoShape::RightTriangle { inner } => inner.area(),
        }
    }

    fn perimeter(&self) -> f64 {
        match *&self {
            MonoShape::Circle { inner } => inner.perimeter(),
            MonoShape::Square { inner } => inner.perimeter(),
            MonoShape::Triangle { inner } => inner.perimeter(),
            MonoShape::EquilateralTriangle { inner } => inner.perimeter(),
            MonoShape::RightTriangle { inner } => inner.perimeter(),
        }
    }
}

impl std::fmt::Display for MonoShape {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *&self {
            MonoShape::Circle { inner } => {
                write!(f, "{}", &inner)
            }
            MonoShape::Square { inner } => {
                write!(f, "{}", &inner)
            }
            MonoShape::Triangle { inner } => {
                write!(f, "{}", &inner)
            }
            MonoShape::EquilateralTriangle { inner } => {
                write!(f, "{}", &inner)
            }
            MonoShape::RightTriangle { inner } => {
                write!(f, "{}", &inner)
            }
        }
    }
}

impl From<Circle> for MonoShape {
    fn from(shape: Circle) -> Self {
        MonoShape::Circle { inner: shape }
    }
}

impl From<Square> for MonoShape {
    fn from(shape: Square) -> Self {
        MonoShape::Square { inner: shape }
    }
}

impl From<Triangle> for MonoShape {
    fn from(shape: Triangle) -> Self {
        MonoShape::Triangle { inner: shape }
    }
}

impl From<EquilateralTriangle> for MonoShape {
    fn from(shape: EquilateralTriangle) -> Self {
        MonoShape::EquilateralTriangle { inner: shape }
    }
}

impl From<RightTriangle> for MonoShape {
    fn from(shape: RightTriangle) -> Self {
        MonoShape::RightTriangle { inner: shape }
    }
}

//------------------------------------------------------------------------------
// Mono Factory Implementation
//------------------------------------------------------------------------------
type MonoDefaultFunction = dyn Fn() -> MonoShape;
type MonoDimFunction = dyn Fn(&[f64]) -> Result<MonoShape, CreationError>;
type MonoShapeTuple<'a> = (&'a str, Box<MonoDefaultFunction>, Box<MonoDimFunction>);

#[rustfmt::skip]
const CREATE_MONOSHAPE: LazyCell<Vec<MonoShapeTuple>> = LazyCell::new(|| {
    vec![
        (
            "Triangle",
            Box::new(|| Triangle::default().into()),
            Box::new(|dims| Ok(Triangle::try_from(dims)?.into())),
        ),
        (
            "Right Triangle",
            Box::new(|| RightTriangle::default().into()),
            Box::new(|dims| Ok(RightTriangle::try_from(dims)?.into())),
        ),
        (
            "Equilateral Triangle",
            Box::new(|| EquilateralTriangle::default().into()),
            Box::new(|dims| Ok(EquilateralTriangle::try_from(dims)?.into())),
        ),
        (
            "Square",
            Box::new(|| Square::default().into()),
            Box::new(|dims| Ok(Square::try_from(dims)?.into())),
        ),
        (
            "Circle",
            Box::new(|| Circle::default().into()),
            Box::new(|dims| Ok(Circle::try_from(dims)?.into())),
        ),
    ]
});

pub struct MonoFactory;

impl CreationFactory for MonoFactory {
    type Item = MonoShape;
    type Error = CreationError;

    /// Create a Shape
    ///
    /// # Arguments
    ///
    ///   * `name` shape to be created
    ///
    fn create_default(name: &str) -> Result<Self::Item, Self::Error> {
        match CREATE_MONOSHAPE
            .iter()
            .find(|(shape_name, _, _)| shape_name == &name)
        {
            Some((_, creation_op, _)) => Ok(creation_op()),
            None => Err(CreationError::UnknownShapeError(name.to_owned())),
        }
    }

    /// Create a Shape with specified dimensions.
    ///
    /// # Arguments
    ///
    ///   * `name` shape to be created
    ///   * `dims` input dimensions
    ///
    fn create_with(name: &str, dims: &[f64]) -> Result<Self::Item, Self::Error> {
        match CREATE_MONOSHAPE
            .iter()
            .find(|(shape_name, _, _)| shape_name == &name)
        {
            None => Err(CreationError::UnknownShapeError(name.to_owned())),
            Some((_, _, creation_op)) => creation_op(&dims),
        }
    }
}

impl FactoryDirectory for MonoFactory {
    /// Determine whether a given shape is known
    ///
    /// # Arguments
    ///
    ///  * `name` the shape for which to query
    ///
    fn is_known(name: &str) -> bool {
        CREATE_MONOSHAPE
            .iter()
            .find(|(shape_name, _, _)| shape_name == &name)
            .is_some()
    }

    fn number_known() -> usize {
        CREATE_MONOSHAPE.len()
    }

    /// List the known shapes, one per line
    ///
    fn list_known() -> String {
        CREATE_MONOSHAPE
            .iter()
            .map(|(name, _, _)| format!("  {}", name))
            .join("\n")
            + "\n"
    }
}
