use std::fmt;

use crate::error::EnrollError;
use crate::roster::Roster;
use crate::student::Student;

pub enum EnrollResult {
    Success {
        stu_name: String,
        course_num: String,
    },
    Failure {
        stu_name: String,
        course_num: String,
        reason: EnrollError,
    },
}

impl fmt::Display for EnrollResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EnrollResult::Success {
                stu_name,
                course_num,
            } => {
                write!(f, "{stu_name:<18} {:>12} in {course_num}", "enrolled")
            }
            EnrollResult::Failure {
                stu_name,
                course_num,
                reason,
            } => {
                write!(
                    f,
                    "{stu_name:<18} {:>12} in {course_num} ({})",
                    "NOT enrolled",
                    match reason {
                        EnrollError::AlreadyRegistered => "Already Enrolled",
                        EnrollError::SectionFull { .. } => "Full",
                        _ => "Unknown Error",
                    }
                )
            }
        }
    }
}

pub fn enroll_everyone(
    mut roster: Roster,
    all_students: impl std::iter::IntoIterator<Item = Student>,
) -> (Vec<EnrollResult>, Roster) {
    let course_num = roster.course_num.clone();

    let messages = all_students
        .into_iter()
        .map(|stu| (stu.name.clone(), stu))
        .map(|(name, stu)| match roster.enroll(stu) {
            Ok(_) => EnrollResult::Success {
                stu_name: name,
                course_num: course_num.clone(),
            },
            Err(roster_error) => EnrollResult::Failure {
                stu_name: name,
                course_num: course_num.clone(),
                reason: roster_error.the_error,
            },
        })
        .collect();

    (messages, roster)
}
