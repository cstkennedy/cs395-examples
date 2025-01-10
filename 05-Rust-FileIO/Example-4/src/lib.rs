pub mod error;
pub mod parser;
pub mod register;
pub mod roster;
pub mod student;

pub mod prelude {
    pub use crate::parser::Parser;
    pub use crate::register;
    pub use crate::roster::Roster;
    pub use crate::student::Student;
}
