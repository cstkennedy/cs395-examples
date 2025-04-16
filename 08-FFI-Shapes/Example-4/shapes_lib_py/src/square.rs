use pyo3::prelude::*;
use pyo3::types::PyAny;

use shapes_lib::prelude::*;
use shapes_lib::square::Square;

#[pyclass(name = "Square")]
#[derive(Clone, Debug, PartialEq)]
pub struct SquareWrapper {
    wrapped: Square,
}

impl From<Square> for SquareWrapper {
    fn from(wrapped: Square) -> Self {
        SquareWrapper { wrapped }
    }
}

#[pymethods]
impl SquareWrapper {
    #[new]
    #[pyo3(signature = (side=1.0))]
    pub fn new(side: f64) -> PyResult<SquareWrapper> {
        Ok(Square::new(side).into())
    }

    #[getter]
    pub fn name(&self) -> &'static str {
        "Square"
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
