use enroll_students::prelude::{Roster, Student};

fn main() {
    let john = Student::new("John");
    let tom = Student::new("Tom");
    let jay = Student::new("Jay");
    let oscar = Student::new("Oscar");

    let all_students = [john, tom, jay, oscar];

    let mut cs330 = Roster::new(3, "CS 330");

    for stu in all_students.into_iter() {
        let course_num = cs330.course_num;
        let name = stu.name;

        /*
        if cs330.enroll(stu) {
            println!("{name} enrolled in {course_num}");
        } else {
            println!("{name} NOT enrolled in {course_num}");
        }
        */
        /*
        match cs330.enroll(stu) {
            true => println!("{name} enrolled in {course_num}"),
            false => println!("{name} NOT enrolled in {course_num}")
        }
        */
        let message = match cs330.enroll(stu) {
            true => format!("{name} enrolled in {course_num}"),
            false => format!("{name} NOT enrolled in {course_num}")
        };
        println!("{}", message);
    }

    println!();
    println!("{cs330}");
}
