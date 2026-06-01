use clap::Parser;
use eyre::WrapErr;

use enroll_students::prelude::{Parser as RosterParser, Roster, register};

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// student filename
    student_filename: String,

    /// roster filename
    roster_filename: String,
}

fn main() -> eyre::Result<()> {
    let args = Args::parse();

    let all_students = RosterParser::read_from_file(&args.student_filename, |ins| {
        RosterParser::read_students(ins)
    })?;
    let all_rosters =
        RosterParser::read_from_file(&args.roster_filename, |ins| RosterParser::read_rosters(ins))?;

    let (combined_messages, populated_rosters): (Vec<Vec<register::EnrollResult>>, Vec<Roster>) =
        all_rosters
            .into_iter()
            .map(|roster| register::enroll_everyone(roster, all_students.clone()))
            .unzip();

    let all_messages = combined_messages.into_iter().flatten();

    for message in all_messages {
        println!("{message}");
    }

    for roster in populated_rosters {
        println!();
        println!("{roster}");
    }

    Ok(())
}
