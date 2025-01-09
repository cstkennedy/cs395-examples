use enroll_students::error::{EnrollError, RosterError};
use enroll_students::prelude::{Roster, Student};

fn main() {
    let all_students = [
        Student::new("John"),
        Student::new("Tom"),
        Student::new("Jay"),
        Student::new("Oscar"),
    ];

    let mut cs330 = Roster::new(3, "CS 330");

    let course_num = cs330.course_num.clone();
    for stu in all_students.into_iter() {
        let name = stu.name.clone();

        let message = match cs330.enroll(stu) {
            Ok(_) => format!("{name} enrolled in {course_num}"),
            Err(enroll_error) => [
                format!("{name} NOT enrolled in {course_num}"),
                format!("  {}", enroll_error),
            ]
            .join("\n"),
        };
        println!("{}", message);
    }

    println!();
    println!("{cs330}");
}
