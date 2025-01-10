use thiserror::Error;

use crate::student::Student;

#[derive(Debug, Error, PartialEq)]
pub enum EnrollError {
    #[error("{course_num} has reached its cap ({cap})")]
    SectionFull { course_num: String, cap: usize },
    #[error("Student is already registered")]
    AlreadyRegistered,
    #[error("{0:?}")]
    Generic(&'static str),
}

#[derive(Debug, Error, PartialEq)]
pub struct ErrorWithValue<E: std::error::Error, V> {
    #[source]
    pub the_error: E,
    pub the_value: V,
}

pub type RosterError = ErrorWithValue<EnrollError, Student>;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("{0:?}")]
    FileNotFound(#[from] std::io::Error),
}
