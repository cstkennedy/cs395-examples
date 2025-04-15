use pyo3::prelude::*;
use pyo3::types::PyAny;

use shapes_lib::equilateral_triangle::EquilateralTriangle;
use shapes_lib::prelude::*;
use shapes_lib::right_triangle::RightTriangle;
use shapes_lib::triangle::Triangle;

#[pyclass(name = "Triangle")]
#[derive(Clone, Debug, PartialEq)]
pub struct TriangleWrapper {
    wrapped: Triangle,
}

impl From<Triangle> for TriangleWrapper {
    fn from(wrapped: Triangle) -> Self {
        TriangleWrapper { wrapped }
    }
}

#[pymethods]
impl TriangleWrapper {
    #[new]
    #[pyo3(signature = (a=1.0, b=1.0, c=1.0))]
    pub fn new(a: f64, b: f64, c: f64) -> PyResult<TriangleWrapper> {
        Ok(Triangle::new(a, b, c).into())
    }

    #[getter]
    pub fn name(&self) -> &'static str {
        "Triangle"
    }

    #[getter]
    pub fn get_side_a(&self) -> f64 {
        self.wrapped.side_a
    }

    #[setter]
    pub fn set_side_a(&mut self, val: f64) {
        self.wrapped.side_a = val
    }

    #[getter]
    pub fn get_side_b(&self) -> f64 {
        self.wrapped.side_b
    }

    #[setter]
    pub fn set_side_b(&mut self, val: f64) {
        self.wrapped.side_b = val
    }

    #[getter]
    pub fn get_side_c(&self) -> f64 {
        self.wrapped.side_c
    }

    #[setter]
    pub fn set_side_c(&mut self, val: f64) {
        self.wrapped.side_c = val
    }

    pub fn area(&self) -> f64 {
        self.wrapped.area()
    }

    pub fn perimeter(&self) -> f64 {
        self.wrapped.perimeter()
    }

    pub fn __deepcopy__(&self, _memo: &'_ Bound<'_, PyAny>) -> Self {
        self.clone()
    }

    pub fn __str__(&self) -> String {
        format!("{}", self.wrapped)
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.wrapped)
    }
}

#[pyclass(name = "EquilateralTriangle")]
#[derive(Clone, Debug, PartialEq)]
pub struct EquilateralTriangleWrapper {
    wrapped: EquilateralTriangle,
}

impl From<EquilateralTriangle> for EquilateralTriangleWrapper {
    fn from(wrapped: EquilateralTriangle) -> Self {
        EquilateralTriangleWrapper { wrapped }
    }
}

#[pymethods]
impl EquilateralTriangleWrapper {
    #[new]
    #[pyo3(signature = (side=1.0))]
    pub fn new(side: f64) -> PyResult<EquilateralTriangleWrapper> {
        Ok(EquilateralTriangle::new(side).into())
    }

    #[getter]
    pub fn name(&self) -> &'static str {
        "Equilateral Triangle"
    }

    #[getter]
    pub fn get_side(&self) -> f64 {
        self.wrapped.side
    }

    #[setter]
    pub fn set_side(&mut self, val: f64) {
        self.wrapped.side = val
    }

    pub fn area(&self) -> f64 {
        self.wrapped.area()
    }

    pub fn perimeter(&self) -> f64 {
        self.wrapped.perimeter()
    }

    pub fn __deepcopy__(&self, _memo: &'_ Bound<'_, PyAny>) -> Self {
        self.clone()
    }

    pub fn __str__(&self) -> String {
        format!("{}", self.wrapped)
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.wrapped)
    }
}

#[pyclass(name = "RightTriangle")]
#[derive(Clone, Debug, PartialEq)]
pub struct RightTriangleWrapper {
    wrapped: RightTriangle,
}

impl From<RightTriangle> for RightTriangleWrapper {
    fn from(wrapped: RightTriangle) -> Self {
        RightTriangleWrapper { wrapped }
    }
}

#[pymethods]
impl RightTriangleWrapper {
    #[new]
    #[pyo3(signature = (base=1.0, height=1.0))]
    pub fn new(base: f64, height: f64) -> PyResult<RightTriangleWrapper> {
        Ok(RightTriangle::new(base, height).into())
    }

    #[getter]
    pub fn name(&self) -> &'static str {
        "Right Triangle"
    }

    #[getter]
    pub fn get_base(&self) -> f64 {
        self.wrapped.base
    }

    #[setter]
    pub fn set_base(&mut self, val: f64) {
        self.wrapped.base = val
    }

    #[getter]
    pub fn get_height(&self) -> f64 {
        self.wrapped.height
    }

    #[setter]
    pub fn set_height(&mut self, val: f64) {
        self.wrapped.height = val
    }

    #[getter]
    pub fn get_hypotenuse(&self) -> f64 {
        self.wrapped.hypotenuse()
    }

    pub fn area(&self) -> f64 {
        self.wrapped.area()
    }

    pub fn perimeter(&self) -> f64 {
        self.wrapped.perimeter()
    }

    pub fn __deepcopy__(&self, _memo: &'_ Bound<'_, PyAny>) -> Self {
        self.clone()
    }

    pub fn __str__(&self) -> String {
        format!("{}", self.wrapped)
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.wrapped)
    }
}
