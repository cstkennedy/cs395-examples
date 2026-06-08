pub mod error;
pub mod parser;
pub mod register;
pub mod roster;
pub mod student;

pub mod prelude {
    pub use crate::parser::RosterParser;
    pub use crate::parser::StudentParser;
    pub use crate::register;
    pub use crate::roster::EmptyRoster;
    pub use crate::roster::PopulatedRosters;
    pub use crate::roster::Roster;
    pub use crate::student::Student;
}
