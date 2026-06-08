use clap::{Parser, ValueEnum};
use eyre::WrapErr;

use itertools::Itertools; // avoid temp Vec<String> when using join
use log;
use serde_toon;
use serde_xml_rs;
use toml;

use enroll_students::prelude::{Parser as RosterParser, Roster, Student, register, PopulatedRosters};

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

    let all_students =
        RosterParser::from_file(&args.student_filename, RosterParser::read_students)?;

    let all_rosters = match args.roster_format {
        FileFormat::Text => {
            RosterParser::from_file(&args.roster_filename, RosterParser::read_rosters)?
        }
        FileFormat::Toml => RosterParser::read_rosters_from_toml(&args.roster_filename)?,
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

    let populated_rosters = PopulatedRosters::from_iter(populated_rosters);

    /*
    match args.output_format {
        OutputFormat::Text => {
            println!(
                "{}",
                populated_rosters.iter().map(Roster::to_string).join("\n")
            );
        }
        OutputFormat::Toml => {
            let rosters_as_toml = toml::to_string(&populated_rosters)?;
            println!("{}", rosters_as_toml);
        }
        OutputFormat::Yaml => {
            println!("{:#}", serde_yml::to_string(&populated_rosters)?,);
        }
        OutputFormat::Json => {
            println!("{:#}", serde_json::to_string_pretty(&populated_rosters)?,);
        }
        OutputFormat::Ron => {
            println!(
                "{}",
                ron::ser::to_string_pretty(&populated_rosters, ron::ser::PrettyConfig::default())?,
            );
        }
        OutputFormat::Toon => {
            println!("{}", serde_toon::to_string_pretty(&populated_rosters)?);
        }
        OutputFormat::Xml => {
            println!("{}", serde_xml_rs::to_string(&populated_rosters)?);
        }
    }
    */

    /*
    let rosters_as_string = match args.output_format {
        OutputFormat::Text => {
            populated_rosters.iter().map(Roster::to_string).join("\n")
        }
        OutputFormat::Toml => {
            toml::to_string(&populated_rosters)?
        }
        OutputFormat::Yaml => {
            serde_yml::to_string(&populated_rosters)?
        }
        OutputFormat::Json => {
            serde_json::to_string_pretty(&populated_rosters)?
        }
        OutputFormat::Ron => {
            ron::ser::to_string_pretty(&populated_rosters, ron::ser::PrettyConfig::default())?
        }
        OutputFormat::Toon => {
            serde_toon::to_string_pretty(&populated_rosters)?
        }
        OutputFormat::Xml => {
            serde_xml_rs::to_string(&populated_rosters)?
        }
    };
    */

    if let Ok(rosters_as_string) = serialize_to_string(&populated_rosters, &args.output_format) {
        println!("{}", rosters_as_string);
    }

    Ok(())
}

fn serialize_to_string(populated_rosters: &PopulatedRosters, output_format: &OutputFormat) -> eyre::Result<String> {
    let serialized_str = match output_format {
        OutputFormat::Text => {
            populated_rosters.iter().map(Roster::to_string).join("\n")
        }
        OutputFormat::Toml => {
            toml::to_string(&populated_rosters)?
        }
        OutputFormat::Yaml => {
            serde_yml::to_string(&populated_rosters)?
        }
        OutputFormat::Json => {
            serde_json::to_string_pretty(&populated_rosters)?
        }
        OutputFormat::Ron => {
            ron::ser::to_string_pretty(&populated_rosters, ron::ser::PrettyConfig::default())?
        }
        OutputFormat::Toon => {
            serde_toon::to_string_pretty(&populated_rosters)?
        }
        OutputFormat::Xml => {
            serde_xml_rs::to_string(&populated_rosters)?
        }
    };

    Ok(serialized_str)

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
