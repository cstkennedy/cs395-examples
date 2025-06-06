use std::str::FromStr;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use itertools::Itertools;

use crate::error::{HouseError, ParseError, RoomError};
use crate::flooring::Flooring;
use crate::house::House;
use crate::room::Room;

const MIN_LINEAR_DIM: f64 = 0.0_f64;
const MIN_COST: f64 = 0.01_f64;
const MIN_NUM_ROOM_TOKENS: usize = 4;
const REQUIRED_NUM_DIMS: usize = 3;

impl FromStr for Room {
    type Err = RoomError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // Separate the name and "the rest"
        let (name, the_rest) = {
            let tokens = line.split(";").collect::<Vec<&str>>();

            if tokens.len() < 2 {
                return Err(ParseError::MissingDelimiter {
                    delim: ";".into(),
                    line: line.to_owned(),
                }
                .into());
            }

            // 1. Grab the name
            // 2. Split everything else by whitespace
            let name = tokens[0];
            let the_rest: Vec<&str> = tokens[1].split_whitespace().collect();

            if the_rest.len() < MIN_NUM_ROOM_TOKENS {
                return Err(ParseError::TooFewTokens {
                    num_tokens: the_rest.len(),
                    line: line.to_owned(),
                }
                .into());
            }

            (name, the_rest)
        };

        // Separate the flooring type and length, width, and flooring cost
        let (length, width, flooring_name, unit_cost) = {
            let nums: Vec<f64> = the_rest[0..3]
                .iter()
                .flat_map(|token| token.parse())
                .collect();

            if nums.len() < REQUIRED_NUM_DIMS {
                return Err(RoomError::from(ParseError::MalformedLine(line.to_owned())));
            }

            // The flooring name might contain spaces.
            // Combine the remainder of the line.
            let flooring_name = the_rest.into_iter().skip(3).join(" ");

            (nums[0], nums[1], flooring_name, nums[2])
        };

        if unit_cost < MIN_COST {
            return Err(RoomError::InvalidCost(MIN_COST));
        }

        let room = Room::builder()
            .with_name(name)
            .with_dimensions(length, width)?
            .with_flooring(
                Flooring::builder()
                    .type_name(flooring_name)
                    .unit_cost(unit_cost)
                    .build(),
            )
            .build();

        Ok(room)
    }
}

pub struct HouseParser;

impl HouseParser {
    /// Open a file and read in data based on a supplied closure
    ///
    /// # Arguments
    ///
    ///   * `filename` - file from which to read
    ///   * `parse_fn` - parsing function to use
    pub fn read_from_file<T, F>(filename: &str, parse_fn: F) -> Result<T, ParseError>
    where
        F: Fn(BufReader<File>) -> T,
    {
        let file = File::open(filename)?;
        let ins = BufReader::new(file);
        let all_things = parse_fn(ins);

        Ok(all_things)
    }

    pub fn read_house(room_data: impl BufRead) -> Result<House, HouseError> {
        let house = House::builder()
            .with_rooms(
                room_data
                    .lines()
                    .flatten()
                    .filter(|line| !line.is_empty())
                    .flat_map(|line| Room::from_str(&line))
                    .collect(),
            )?
            .build();

        Ok(house)
    }
}
