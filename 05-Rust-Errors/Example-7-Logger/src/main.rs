use eyre::{OptionExt, Result, WrapErr};
use log;

use simple_logger::SimpleLogger;

use enroll_students::error::{EnrollError, RosterError};
use enroll_students::prelude::{Roster, Student};

fn main() -> Result<()> {
    SimpleLogger::new().env().init().unwrap();

    let all_students = [
        Student::new("John"),
        Student::new("Tom"),
        Student::new("Jay"),
        Student::new("Oscar"),
    ];

    let arg: String = std::env::args()
        .nth(1)
        .ok_or_eyre("No capacity was provided")?;

    let cap = arg
        .parse()
        .wrap_err_with(|| format!("'{arg}' is not a valid usize"))?;

    let cs330 = enroll_everyone(Roster::new(cap, "CS 330"), all_students);

    println!();
    println!("{cs330}");

    Ok(())
}

fn enroll_everyone(
    mut roster: Roster,
    all_students: impl std::iter::IntoIterator<Item = Student>,
) -> Roster {
    let course_num = roster.course_num.clone();

    all_students
        .into_iter()
        .for_each(|stu| {
            let name = stu.name.clone();

            match roster.enroll(stu) {
                Ok(_) => log::info!("{name} enrolled in {course_num}"),
                Err(roster_error) => {
                    log::warn!(
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
        });

    roster
}
