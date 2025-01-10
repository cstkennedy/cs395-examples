use std::fmt;

use eyre::WrapErr;

use enroll_students::error::{EnrollError, RosterError};
use enroll_students::prelude::{Parser, Roster, Student};

fn main() -> eyre::Result<()> {
    let argv: Vec<String> = std::env::args().collect();

    if argv.len() < 3 {
        eyre::bail!("Usage: {} student_filename roster_filename", argv[0]);
    }

    let all_students = Parser::read_from_file(&argv[1], |ins| Parser::read_students(ins))?;
    let all_rosters = Parser::read_from_file(&argv[2], |ins| Parser::read_rosters(ins))?;

    let populated_rosters: Vec<Roster> = all_rosters
        .into_iter()
        .map(|roster| enroll_everyone(roster, all_students.clone()))
        .map(|(logged_messages, roster)| {
            for message in logged_messages {
                println!("{}", message);
            }

            roster
        })
        .collect();

    for roster in populated_rosters {
        println!();
        println!("{roster}");
    }

    Ok(())
}

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

fn enroll_everyone(
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
