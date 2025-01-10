use std::fs::File;
use std::io::BufReader;

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

fn enroll_everyone(
    mut roster: Roster,
    all_students: impl std::iter::IntoIterator<Item = Student>,
) -> (Vec<String>, Roster) {
    let course_num = roster.course_num.clone();

    let messages = all_students
        .into_iter()
        .map(|stu| (stu.name.clone(), stu))
        .map(|(name, stu)| match roster.enroll(stu) {
            Ok(_) => format!("{name} enrolled in {course_num}"),
            Err(roster_error) => {
                format!(
                    "{} NOT enrolled in {course_num} ({})",
                    roster_error.the_value,
                    match roster_error.the_error {
                        EnrollError::AlreadyRegistered => "Already Enrolled",
                        EnrollError::SectionFull { .. } => "Full",
                        _ => "Unknown Error",
                    }
                )
            }
        })
        .collect();

    (messages, roster)
}
