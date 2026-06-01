use clap::Parser;
use eyre::WrapErr;

use enroll_students::prelude::{Parser as RosterParser, Roster, Student, register};

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

    let all_students =
        RosterParser::from_file(&args.student_filename, RosterParser::read_students)?;
    let all_rosters = RosterParser::from_file(&args.roster_filename, RosterParser::read_rosters)?;

    /*
    let (combined_messages, populated_rosters): (Vec<Vec<register::EnrollResult>>, Vec<Roster>) =
        all_rosters
            .into_iter()
            .map(|roster| register::enroll_everyone(roster, all_students.clone()))
            .unzip();
    */

    let (combined_messages, populated_rosters) = populate_all(all_rosters, all_students);

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
