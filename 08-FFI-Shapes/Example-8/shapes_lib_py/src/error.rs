use pyo3::exceptions;
use pyo3::prelude::PyErr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShapeCreationError {
    #[error("{0:?} is not known")]
    UnknownShapeError(String),

    #[error("{name:?} requires '{num_required}' dimension(s)")]
    DimensionCountError { name: String, num_required: usize },

    #[error("{0}")]
    MalformedLineError(String),
}

impl From<ShapeCreationError> for PyErr {
    fn from(err: ShapeCreationError) -> Self {
        match err {
            ShapeCreationError::UnknownShapeError(_) => {
                exceptions::PyKeyError::new_err(err.to_string())
            }
            ShapeCreationError::DimensionCountError { .. }
            | ShapeCreationError::MalformedLineError(_) => {
                exceptions::PyValueError::new_err(err.to_string())
            }
        }
    }
}
