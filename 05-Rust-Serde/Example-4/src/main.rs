use clap::{Parser, ValueEnum};
use eyre::WrapErr;

use itertools::Itertools; // avoid temp Vec<String> when using join
use log;
use serde_toon;
use serde_xml_rs;
use toml;

use enroll_students::parser;
use enroll_students::prelude::{EmptyRoster, EnrollResult, PopulatedRosters, Roster, Student};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum FileFormat {
    Text,
    Toml,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum OutputFormat {
    Text,
    Toml,
    Yaml,
    Json,
    Ron,
    Toon,
    Xml,
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
    output_format: OutputFormat,
}

fn main() -> eyre::Result<()> {
    let args = Args::parse();

    let all_students = parser::StudentParser::read_from_text(&args.student_filename)?;

    let all_rosters: Vec<EmptyRoster> = match args.roster_format {
        FileFormat::Text => parser::RosterParser::read_from_text(&args.roster_filename)?,
        FileFormat::Toml => parser::RosterParser::read_from_toml(&args.roster_filename)?,
    };

    if all_rosters.len() == 0 {
        eyre::bail!("There must be at least one valid roster");
    }

    let (combined_messages, populated_rosters) = populate_all(all_rosters, all_students);

    let all_messages = combined_messages.into_iter().flatten();

    for message in all_messages {
        log::info!("{message}");
    }

    let populated_rosters = PopulatedRosters::from_iter(populated_rosters);

    if let Ok(rosters_as_string) = serialize_to_string(&populated_rosters, &args.output_format) {
        println!("{}", rosters_as_string);
    }

    Ok(())
}

fn serialize_to_string(
    populated_rosters: &PopulatedRosters,
    output_format: &OutputFormat,
) -> eyre::Result<String> {
    let serialized_str = match output_format {
        OutputFormat::Text => populated_rosters.iter().map(Roster::to_string).join("\n"),
        OutputFormat::Toml => toml::to_string(&populated_rosters)?,
        OutputFormat::Yaml => serde_yml::to_string(&populated_rosters)?,
        OutputFormat::Json => serde_json::to_string_pretty(&populated_rosters)?,
        OutputFormat::Ron => {
            ron::ser::to_string_pretty(&populated_rosters, ron::ser::PrettyConfig::default())?
        }
        OutputFormat::Toon => serde_toon::to_string_pretty(&populated_rosters)?,
        OutputFormat::Xml => serde_xml_rs::to_string(&populated_rosters)?,
    };

    Ok(serialized_str)
}

pub fn populate_all<R, S>(
    empty_rosters: R,
    all_students: S,
) -> (Vec<Vec<EnrollResult>>, Vec<Roster>)
where
    R: std::iter::IntoIterator<Item = EmptyRoster>,
    S: std::iter::IntoIterator<Item = Student> + Clone,
{
    empty_rosters
        .into_iter()
        // .map(Roster::from)
        // .map(|empty_roster| register::enroll_everyone(empty_roster, all_students.clone()))
        .map(|empty_roster| empty_roster.enroll_everyone(all_students.clone()))
        .unzip()
}
