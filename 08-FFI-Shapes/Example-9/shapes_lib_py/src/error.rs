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

impl From<shapes_lib::error::CreationError> for ShapeCreationError {
    fn from(err: shapes_lib::error::CreationError) -> Self {
        match err {
            shapes_lib::error::CreationError::UnknownShapeError(msg) => {
                ShapeCreationError::UnknownShapeError(msg)
            }

            shapes_lib::error::CreationError::DimensionCountError { name, num_required } => {
                ShapeCreationError::DimensionCountError { name, num_required }
            }

            shapes_lib::error::CreationError::MalformedLineError(msg) => {
                ShapeCreationError::MalformedLineError(msg)
            }
        }
    }
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
