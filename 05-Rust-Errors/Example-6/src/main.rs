use eyre::{WrapErr, Result, OptionExt};

use enroll_students::error::{EnrollError, RosterError};
use enroll_students::prelude::{Roster, Student};

fn main() -> Result<()> {
    let all_students = [
        Student::new("John"),
        Student::new("Tom"),
        Student::new("Jay"),
        Student::new("Oscar"),
    ];

    let arg: String = std::env::args().nth(1).ok_or_eyre("No capacity was provided")?;
    let cap = arg.parse().wrap_err_with(|| format!("'{arg}' is not a valid usize"))?;

    let mut cs330 = Roster::new(cap, "CS 330");
    enroll_everyone(&mut cs330, all_students);

    println!();
    println!("{cs330}");

    Ok(())
}

fn enroll_everyone(
    roster: &mut Roster,
    all_students: impl std::iter::IntoIterator<Item = Student>,
) {
    let course_num = roster.course_num.clone();

    for stu in all_students.into_iter() {
        let name = stu.name.clone();

        let message = match roster.enroll(stu) {
            Ok(_) => format!("{name} enrolled in {course_num}"),
            Err(roster_error) => {
                let student = roster_error.the_value;

                format!(
                    "{} NOT enrolled in {course_num} ({})",
                    student,
                    match roster_error.the_error {
                        EnrollError::AlreadyRegistered => "Already Enrolled",
                        EnrollError::SectionFull { .. } => "Full",
                        _ => "Unknown Error",
                    }
                )
            }
        };
        println!("{}", message);
    }
}
