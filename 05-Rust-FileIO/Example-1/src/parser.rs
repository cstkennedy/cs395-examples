use std::io::BufRead;

use crate::prelude::{Roster, Student};

pub struct Parser;

impl Parser {
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
}
