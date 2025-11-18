use crate::factory::Factory;
use crate::factory::MonoShape;
use crate::error::CreationError;

use std::io::BufRead;
use std::str::FromStr;


impl FromStr for MonoShape {
    type Err = CreationError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let split_line: Vec<&str> = line.trim().split(";").collect();

        if split_line.len() != 2 {
            return Err(CreationError::MalformedLineError(format!(
                "Line '{line}' did not have exactly one (1) semicolon"
            )));
        }

        let name = split_line[0];
        let values: Vec<f64> = split_line[1]
            .split_whitespace()
            .flat_map(|token| token.parse())
            .collect();

        let shape = Factory::create_with(name, &values)?;

        Ok(shape)
    }
}

pub struct Parser;

impl Parser {
    /// Create shapes based on names from an input buffer.
    ///
    /// # Arguments
    ///
    ///  * `ins` - input source
    ///
    pub fn read_shapes<B: BufRead>(ins: B) -> Vec<MonoShape> {
        ins.lines()
            .flatten()
            .flat_map(|line| {
                let name = line.trim();
                Factory::create(name)
            })
            .collect()
    }

    /// Create shapes based on names *and dimension data* from an input buffer.
    ///
    /// # Arguments
    ///
    ///  * `ins` - input source
    ///
    pub fn read_shapes_with<B>(ins: B) -> Vec<MonoShape>
    where
        B: BufRead,
    {
        ins.lines()
            .flatten()
            .flat_map(|line| MonoShape::from_str(&line))
            .collect()
    }
}
