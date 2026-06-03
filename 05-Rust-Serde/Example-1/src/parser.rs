use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::str::FromStr;

use serde::{Serialize, Deserialize};

use crate::error::ParserError;
use crate::prelude::{Roster, Student};

impl FromStr for Student {
    type Err = ParserError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // Trim the line before length check
        let name = line.trim();

        if name.len() < 3 {
            return Err(ParserError::NameTooShortError {
                name: name.to_owned(),
            });
        }

        Ok(Student::new(name))
    }
}

impl FromStr for Roster {
    type Err = ParserError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<_> = line
            .split(";")
            .map(|token| token.trim().to_owned())
            .collect();

        if tokens.len() != 2 {
            return Err(ParserError::WrongNumberOfTokens { tokens });
        }

        let name = tokens[0].to_owned();
        let cap = tokens[1].parse::<usize>().unwrap_or(0);

        if cap == 0 {
            return Err(ParserError::NotPostiveNumberError {
                raw_num: cap.to_string(),
            });
        }

        Ok(Roster::new(cap, &name))
    }
}

pub struct Parser;

impl Parser {
    /// Open a file and read in data based on a supplied closure
    ///
    /// # Arguments
    ///
    ///   * `filename` - file from which to read
    ///   * `parse_fn` - parsing function to use
    pub fn from_file<T, F>(filename: &str, parse_fn: F) -> Result<T, ParserError>
    where
        F: Fn(BufReader<File>) -> T,
    {
        let file = File::open(filename)?;
        let ins = BufReader::new(file);
        let all_things = parse_fn(ins);

        Ok(all_things)
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
            .flat_map(|line| line.parse())
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
            .flat_map(|line| line.parse())
            .collect()
    }

    /// Create rosters from an input buffer.
    ///
    /// # Arguments
    ///
    ///  * `ins` - input source
    ///
    pub fn read_rosters_from_toml(filename: &str) -> Result<Vec<Roster>, ParserError> {
        let mut toml_file = File::open(filename)?;
        let mut toml_str = String::new();

        toml_file.read_to_string(&mut toml_str)?;

        let all_empty_rosters: EmptyRosters = toml::from_str(&toml_str)?;

        // println!("{:#?}", all_empty_rosters);

        let all_rosters = all_empty_rosters.courses
            .into_iter()
            .map(Roster::from)
            .collect();

        Ok(all_rosters)
    }
}

#[derive(Debug, Deserialize)]
pub struct EmptyRoster {
    pub name: String,
    pub capacity: usize
}

impl From<EmptyRoster> for Roster {
    fn from(empty_roster: EmptyRoster) -> Self {
        Self::new(empty_roster.capacity, &empty_roster.name)
    }
}

#[derive(Debug, Deserialize)]
pub struct EmptyRosters {
    #[serde(rename = "course")]
    pub courses: Vec<EmptyRoster>
}
