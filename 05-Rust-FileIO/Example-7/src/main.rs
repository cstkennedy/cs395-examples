use clap::{Parser, ValueEnum};
use eyre::WrapErr;

use log;
use toml;
use serde::Serialize;

use enroll_students::prelude::{Parser as RosterParser, Roster, Student, register};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum FileFormat {
    Text,
    Toml
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum OutputFormat {
    Text,
    Toml
}

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// student filename
    student_filename: String,

    /// roster filename
    roster_filename: String,

    /// roster file format
    #[arg(long, value_enum, default_value_t = FileFormat::Toml)]
    roster_format: FileFormat,

    /// output location and format
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    output_format: OutputFormat
}

#[derive(Serialize)]
pub struct AllRosters {
    rosters: Vec<Roster>,
}

fn main() -> eyre::Result<()> {
    let args = Args::parse();

    let all_students =
        RosterParser::from_file(&args.student_filename, RosterParser::read_students)?;

    let all_rosters = match args.roster_format {
        FileFormat::Text => {
            RosterParser::from_file(&args.roster_filename, RosterParser::read_rosters)?
        },
        FileFormat::Toml => {
            RosterParser::read_rosters_from_toml(&args.roster_filename)?
        }
    };

    if all_rosters.len() == 0 {
        eyre::bail!("There must be at least one valid roster");
    }

    let (combined_messages, populated_rosters) = populate_all(all_rosters, all_students);

    let all_messages = combined_messages.into_iter().flatten();

    // Reinvented logging
    for message in all_messages {
        log::info!("{message}");
    }

    match args.output_format {
        OutputFormat::Text => {
            for roster in populated_rosters.iter() {
                println!();
                print!("{roster}");
            }
        },
        OutputFormat::Toml => {
            let populated_rosters = AllRosters {
                rosters: populated_rosters,
            };

            let rosters_as_toml = toml::to_string(&populated_rosters)?;
            println!("{}", rosters_as_toml);
        }
    }

    Ok(())
}

pub fn populate_all<R, S>(
    all_rosters: R,
    all_students: S,
) -> (Vec<Vec<register::EnrollResult>>, Vec<Roster>)
where
    R: std::iter::IntoIterator<Item = Roster>,
    S: std::iter::IntoIterator<Item = Student> + Clone,
{
    all_rosters
        .into_iter()
        .map(|roster| register::enroll_everyone(roster, all_students.clone()))
        .unzip()
}
