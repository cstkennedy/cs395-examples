use eyre::WrapErr;

use enroll_students::prelude::{register, Parser, Roster};

fn main() -> eyre::Result<()> {
    let argv: Vec<String> = std::env::args().collect();

    if argv.len() < 3 {
        eyre::bail!("Usage: {} student_filename roster_filename", argv[0]);
    }

    let all_students = Parser::read_from_file(&argv[1], |ins| Parser::read_students(ins))?;
    let all_rosters = Parser::read_from_file(&argv[2], |ins| Parser::read_rosters(ins))?;

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
