pub mod error;
pub mod parser;
pub mod roster;
pub mod student;

pub mod prelude {
    pub use crate::roster::EmptyRoster;
    pub use crate::roster::EnrollResult;
    pub use crate::roster::PopulatedRosters;
    pub use crate::roster::Roster;
    pub use crate::student::Student;
}
