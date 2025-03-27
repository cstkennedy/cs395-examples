use std::fs::File;
use std::io::{BufReader, Read};

use clap::{Parser, ValueEnum};
use eyre::{self, WrapErr};
use toml::{Table, Value};
use serde_json;
use serde_yml;

use dnd_serde_example::*;

#[derive(clap::Parser)]
struct Args {
    filename: String,

    #[arg(value_enum, long, short)]
    mode: DemoMode,
}

#[derive(Clone, Debug, ValueEnum)]
enum DemoMode {
    Table,
    Derive,
    ConvertToRon,
    ConvertToJson,
    ConvertToYaml,
}

fn main() -> eyre::Result<()> {
    let cli = Args::parse();
    let filename = &cli.filename;

    let raw_toml_str: String = {
        let mut input = BufReader::new(
            File::open(filename).wrap_err_with(|| format!("Could not open '{}'", filename))?,
        );

        let mut raw_str = String::new();
        input.read_to_string(&mut raw_str)?;

        raw_str
    };

    match cli.mode {
        DemoMode::Table => {
            let value = raw_toml_str.as_str().parse::<Table>().unwrap();
            println!("{:#?}", &value);
            println!();

            for entry in value {
                if let (_, Value::Array(val)) = entry {
                    println!("{:#?}", &val);
                }
                println!();
            }
        }
        DemoMode::Derive => {
            let parsed_char_toml: CharacterTOML = toml::from_str(&raw_toml_str)?;

            println!("{:#?}", parsed_char_toml);
            /*
            for class in parsed_char_toml.iter() {
                println!("{:#?}", class);
            }
            */
        }
        DemoMode::ConvertToRon => {
            let parsed_char_toml: CharacterTOML = toml::from_str(&raw_toml_str)?;

            println!(
                "{}",
                ron::ser::to_string_pretty(&parsed_char_toml, ron::ser::PrettyConfig::default())?,
            );
        }
        DemoMode::ConvertToJson => {
            let parsed_char_toml: CharacterTOML = toml::from_str(&raw_toml_str)?;

            println!(
                "{:#}",
                serde_json::to_string_pretty(&parsed_char_toml)?,
            );
        }
        DemoMode::ConvertToYaml => {
            let parsed_char_toml: CharacterTOML = toml::from_str(&raw_toml_str)?;

            println!(
                "{:#}",
                serde_yml::to_string(&parsed_char_toml)?,
            );
        }
    }

    Ok(())
}
