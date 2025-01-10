use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use eyre::WrapErr;

use enroll_students::error::{EnrollError, RosterError};
use enroll_students::prelude::{Roster, Student};

fn main() -> eyre::Result<()> {
    let argv: Vec<String> = std::env::args().collect();

    if argv.len() < 3 {
        eyre::bail!("Usage: {} student_filename roster_filename", argv[0]);
        // std::process::exit(1);
    }

    let filename: &str = &argv[1];
    let file = File::open(filename).wrap_err_with(|| format!("Could not open '{}", filename))?;
    let ins = BufReader::new(file);
    let all_students = read_students(ins);

    let filename: &str = &argv[2];
    let file = File::open(filename).wrap_err_with(|| format!("Could not open '{}", filename))?;
    let ins = BufReader::new(file);
    let all_rosters = read_rosters(ins);

    let populated_rosters: Vec<Roster> = all_rosters
        .into_iter()
        .map(|roster| {
            let (logged_messages, roster) = enroll_everyone(roster, all_students.clone());

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

/// Read Student names from an input buffer.
///
/// # Arguments
///
///  * `ins` - input source
///
pub fn read_students<B: BufRead>(ins: B) -> Vec<Student> {
    ins.lines()
        .flatten()
        .filter(|line| line.len() > 0)
        .map(|line| {
            let name = line.trim();
            Student::new(name)
        })
        .collect()
}

/// Create rosters from an input buffer.
///
/// # Arguments
///
///  * `ins` - input source
///
pub fn read_rosters<B: BufRead>(ins: B) -> Vec<Roster> {
    ins.lines()
        .flatten()
        .map(|line| {
            line.split(";")
                .map(|token| token.trim().to_owned())
                .collect::<Vec<_>>()
        })
        .filter(|tokens| tokens.len() == 2)
        .map(|tokens| {
            let name = tokens[0].to_owned();
            let cap = tokens[1].parse::<usize>().unwrap_or(0);

            (name, cap)
        })
        .filter(|(_, cap)| *cap > 0_usize)
        .map(|(name, cap)| Roster::new(cap, &name))
        .collect()
}

fn enroll_everyone(
    mut roster: Roster,
    all_students: impl std::iter::IntoIterator<Item = Student>,
) -> (Vec<String>, Roster) {
    let course_num = roster.course_num.clone();

    let messages = all_students
        .into_iter()
        .map(|stu| {
            let name = stu.name.clone();

            match roster.enroll(stu) {
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
            }
        })
        .collect();

    (messages, roster)
}
