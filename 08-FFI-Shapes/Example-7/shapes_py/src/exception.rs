
use pyo3::exceptions;
use pyo3::prelude::PyErr;
use pyo3::prelude::*;
use pyo3::exceptions::PyException;

use crate::error::ShapeConversionError;


#[pyclass(extends = PyException, subclass, name="ShapeConversionError")]
pub struct PyShapeConversionError {
    #[pyo3(get)]
    pub requested: String,

    #[pyo3(get)]
    pub actual: String,
}

#[pymethods]
impl PyShapeConversionError {
    #[new]
    fn new(requested: String, actual: String) -> Self {
        Self {
            requested, actual
        }
    }


    fn __str__(&self) -> String {
        format!("Cannot convert '{}' into '{}'", self.actual, self.requested)
    }
}

impl From<ShapeConversionError> for PyErr {
    fn from(src: ShapeConversionError) -> Self {
        match src {
            ShapeConversionError::TypeMismatchError{ requested, actual } => {
                PyErr::new::<PyShapeConversionError, _>((requested.to_string(), actual.to_string()))
            }
        }
    }
}
