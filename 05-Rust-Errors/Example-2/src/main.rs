use enroll_students::prelude::{Roster, Student};

fn main() {
    let all_students = [
        Student::new("John"),
        Student::new("Tom"),
        Student::new("Jay"),
        Student::new("Oscar"),
    ];

    let mut cs330 = Roster::new(3, "CS 330");

    for stu in all_students.into_iter() {
        let course_num = cs330.course_num.clone();
        let name = stu.name.clone();

        let message = match cs330.enroll(stu) {
            true => format!("{name} enrolled in {course_num}"),
            false => format!("{name} NOT enrolled in {course_num}"),
        };
        println!("{}", message);
    }

    println!();
    println!("{cs330}");
}
