use pyo3::prelude::*;
use pyo3::types::PyAny;

use shapes_lib::circle::Circle;
use shapes_lib::prelude::*;

use crate::utilities;

#[pyclass(name = "Circle")]
#[derive(Clone, Debug, PartialEq)]
pub struct CircleWrapper {
    wrapped: Circle,
}

impl From<Circle> for CircleWrapper {
    fn from(wrapped: Circle) -> Self {
        CircleWrapper { wrapped }
    }
}

#[pymethods]
impl CircleWrapper {
    #[classattr]
    const PI: f64 = std::f64::consts::PI;

    #[classattr]
    const TAU: f64 = std::f64::consts::TAU;

    #[new]
    #[pyo3(signature = (radius=1.0))]
    pub fn new(radius: f64) -> PyResult<CircleWrapper> {
        Ok(Circle::new(radius).into())
    }

    #[getter]
    pub fn name(&self) -> &'static str {
        "Circle"
    }

    #[getter]
    pub fn get_radius(&self) -> f64 {
        self.wrapped.radius
    }

    #[setter]
    pub fn set_radius(&mut self, val: f64) {
        self.wrapped.radius = val
    }

    #[getter]
    pub fn get_diameter(&self) -> f64 {
        self.wrapped.diameter()
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
        utilities::py_repr_from_debug(&self.wrapped)
    }
}
