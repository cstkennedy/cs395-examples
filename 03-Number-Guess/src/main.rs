use std::io::{stdin, stdout};
use std::io::Write;

fn main() {
    let mut lower_limit = 0_u32;
    let mut upper_limit = 100_u32;

    loop {
        let next_guess = get_next_guess(lower_limit, upper_limit);

        print!("Is your number {next_guess}? (y/n): ");
        let _ = stdout().flush();
        let answer = read_line_as_string().to_lowercase();

        if &answer == "y" {
            println!("I win!");
            break;
        }
        else {
            print!("Was my guess too high or too low? (h/l): ");
            let _ = stdout().flush();
            let reply = read_line_as_string().to_lowercase();

            // if too high {
            if &reply == "h" {
                upper_limit = next_guess;
            }
            else {
                lower_limit = next_guess;
            }
        }
    }
}

pub fn get_next_guess(lower_limit: u32, upper_limit: u32) -> u32 {
    lower_limit + (upper_limit - lower_limit) / 2_u32
}

/// Read a line from std in and trim the trailing newline.
fn read_line_as_string() -> String {
    let mut str_buffer = String::new();
    let _ = stdin().read_line(&mut str_buffer).unwrap();
    let str_buffer = str_buffer.trim().to_string();

    str_buffer
}
