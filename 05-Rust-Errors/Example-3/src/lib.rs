pub mod roster;
pub mod student;
pub mod error;

pub mod prelude {
    pub use crate::roster::Roster;
    pub use crate::student::Student;
}
