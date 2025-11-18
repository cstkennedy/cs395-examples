use crate::circle::Circle;
use crate::equilateral_triangle::EquilateralTriangle;
use crate::right_triangle::RightTriangle;
use crate::shape::Shape;
use crate::square::Square;
use crate::triangle::Triangle;

use std::cell::LazyCell;
use std::fmt;

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

impl fmt::Display for MonoShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

impl From<&[f64]> for Triangle {
    fn from(dims: &[f64]) -> Self {
        Triangle {
            side_a: dims[0],
            side_b: dims[1],
            side_c: dims[2],
        }
    }
}

impl From<&[f64]> for EquilateralTriangle {
    fn from(dims: &[f64]) -> Self {
        EquilateralTriangle { side: dims[0] }
    }
}

impl From<&[f64]> for RightTriangle {
    fn from(dims: &[f64]) -> Self {
        RightTriangle {
            base: dims[0],
            height: dims[1],
        }
    }
}

impl From<&[f64]> for Circle {
    fn from(dims: &[f64]) -> Self {
        Circle { radius: dims[0] }
    }
}

impl From<&[f64]> for Square {
    fn from(dims: &[f64]) -> Self {
        Square { side: dims[0] }
    }
}

type DefaultFunction = dyn Fn() -> MonoShape;
type DimFunction = dyn Fn(&[f64]) -> MonoShape;
type ShapeTuple<'a> = (&'a str, Box<DefaultFunction>, Box<DimFunction>, usize);

#[rustfmt::skip]
const CREATE_SHAPE: LazyCell<Vec<ShapeTuple>> = LazyCell::new(|| {
    vec![
        (
            "Triangle",
            Box::new(|| Triangle::default().into()),
            Box::new(|dims| Triangle::from(dims).into()),
            3
        ),
        (
            "Right Triangle",
            Box::new(|| RightTriangle::default().into()),
            Box::new(|dims| RightTriangle::from(dims).into()),
            2,
        ),
        (
            "Equilateral Triangle",
            Box::new(|| EquilateralTriangle::default().into()),
            Box::new(|dims| EquilateralTriangle::from(dims).into()),
            1
        ),
        (
            "Square",
            Box::new(|| Square::default().into()),
            Box::new(|dims| Square::from(dims).into()),
            1
        ),
        (
            "Circle",
            Box::new(|| Circle::default().into()),
            Box::new(|dims| Circle::from(dims).into()),
            1
        ),
    ]
});

pub struct Factory;

impl Factory {
    /// Create a Shape
    ///
    /// # Arguments
    ///
    ///   * `name` shape to be created
    ///
    pub fn create(name: &str) -> Option<MonoShape> {
        match CREATE_SHAPE
            .iter()
            .find(|(shape_name, _, _, _)| shape_name == &name)
        {
            Some((_, creation_op, _, _)) => creation_op().into(),
            _ => None,
        }
    }

    /// Create a Shape with specified dimensions.
    ///
    /// # Arguments
    ///
    ///   * `name` shape to be created
    ///   * `dims` input dimensions
    ///
    pub fn create_with(name: &str, dims: &[f64]) -> Option<MonoShape> {
        if let Some((_, _, creation_op, required_len)) = CREATE_SHAPE
            .iter()
            .find(|(shape_name, _, _, _)| shape_name == &name)
        {
            if *required_len == dims.len() {
                return creation_op(&dims).into();
            }
        }
        None
    }

    /// Determine whether a given shape is known
    ///
    /// # Arguments
    ///
    ///  * `name` the shape for which to query
    ///
    pub fn is_known(name: &str) -> bool {
        CREATE_SHAPE
            .iter()
            .find(|(shape_name, _, _, _)| shape_name == &name)
            .is_some()
    }

    pub fn number_known() -> usize {
        CREATE_SHAPE.len()
    }

    /// List the known shapes, one per line
    ///
    pub fn list_known() -> String {
        CREATE_SHAPE
            .iter()
            .map(|(name, _, _, _)| format!("  {}", name))
            .collect::<Vec<String>>()
            .join("\n")
            + "\n"
    }
}
