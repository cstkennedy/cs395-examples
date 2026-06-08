use std::fmt;

use serde::{Deserialize, Serialize};

use crate::error::{EnrollError, RosterError};
use crate::student::Student;

pub const DEFAULT_MAX_STUDENTS: usize = 10;

#[derive(Debug, Deserialize)]
pub struct EmptyRoster {
    pub name: String,
    pub capacity: usize,
}

impl EmptyRoster {
    pub fn new<'a>(limit: usize, num: &'a str) -> Self {
        Self {
            name: num.to_owned(),
            capacity: limit,
        }
    }
}

impl From<EmptyRoster> for Roster {
    fn from(empty_roster: EmptyRoster) -> Self {
        Roster {
            course_num: empty_roster.name,
            enroll_limit: empty_roster.capacity,
            students: Default::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct EmptyRosters {
    #[serde(rename = "course")]
    pub courses: Vec<EmptyRoster>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct Roster {
    pub course_num: String,
    pub enroll_limit: usize,
    students: Vec<Student>,
}

impl Default for Roster {
    fn default() -> Self {
        Roster {
            course_num: String::from("CS 150"),
            enroll_limit: DEFAULT_MAX_STUDENTS,
            students: Default::default(),
        }
    }
}

impl Roster {
    pub fn enroll<'a>(&'a mut self, stu: Student) -> Result<(), RosterError> {
        if self.students.len() == self.enroll_limit {
            return Err(RosterError {
                the_error: EnrollError::SectionFull {
                    course_num: self.course_num.clone(),
                    cap: self.enroll_limit,
                },
                the_value: stu,
            });
        }

        if self.students.contains(&stu) {
            return Err(RosterError {
                the_error: EnrollError::AlreadyRegistered,
                the_value: stu,
            });
        }

        self.students.push(stu);
        Ok(())
    }

    pub fn list_enrolled_students(&self) -> &Vec<Student> {
        &self.students
    }

    pub fn num_enrolled(&self) -> usize {
        self.students.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Student> {
        self.students.iter()
    }
}

impl fmt::Display for Roster {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // BUG: Integer arithmetic
        // let percent_full = 100.0 * (self.num_enrolled() as f64 / self.enroll_limit) as f64;
        let percent_full = 100.0 * (self.num_enrolled() as f64 / self.enroll_limit as f64);

        writeln!(
            f,
            "{} -> {} of {} ({:.0}% full)",
            self.course_num,
            self.num_enrolled(),
            self.enroll_limit,
            percent_full
        )?;

        for stu in self.students.iter() {
            writeln!(f, "  - {}", stu)?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct PopulatedRosters {
    rosters: Vec<Roster>,
}

impl PopulatedRosters {
    pub fn from_iter<I>(rosters: I) -> Self
    where
        I: std::iter::IntoIterator<Item = Roster>,
    {
        Self {
            rosters: rosters.into_iter().collect(),
        }
    }

    pub fn iter(&self) -> impl std::iter::Iterator<Item = &Roster> {
        self.rosters.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest2::prelude::*;

    #[test]
    fn test_default_constructor() {
        let default_course = Roster::default();

        assert_that!(&default_course.course_num, equal_to("CS 150"));
        assert_that!(default_course.enroll_limit, equal_to(DEFAULT_MAX_STUDENTS));

        assert_that!(default_course.num_enrolled(), equal_to(0));

        // No students have been added
        assert_that!(default_course.list_enrolled_students().len(), equal_to(0));

        // skipping hashcode
        // skipping equals

        // test __str__
        let default_course_str = default_course.to_string();
        assert_that!(default_course_str.contains("CS 150"), is(true));
        assert_that!(
            default_course_str.contains(&default_course.num_enrolled().to_string()),
            is(true)
        );
        assert_that!(
            default_course_str.contains(&DEFAULT_MAX_STUDENTS.to_string()),
            is(true)
        );
    }
}
