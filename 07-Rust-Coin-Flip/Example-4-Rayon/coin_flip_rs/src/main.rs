use std::env;
use std::time::{Duration, Instant};

use itertools::Itertools;

use ::coin_flip::flip_task::FlipTask;
use ::coin_flip::*;

const DEFAULT_NUM_TRIALS: u64 = 10_000;

fn __parse_args<I>(args: I) -> (usize, u64)
where
    I: Iterator<Item = String>,
{
    let args: Vec<String> = args.collect();
    let num_tasks: usize = args[1].parse().unwrap_or(1);

    let num_flips: u64 = args[2]
        .replace(",", "")
        .replace("_", "")
        .parse()
        .unwrap_or(DEFAULT_NUM_TRIALS);

    (num_tasks, num_flips)
}

fn main() {
    let (num_tasks, num_flips) = __parse_args(env::args());
    let start = Instant::now();
    let (overall, results) = if num_tasks == 1 {
        let result = FlipTask::simulate_flips(num_flips);

        (result, vec![result])
    } else {
        run_parallel(num_tasks, num_flips)
    };
    let duration = start.elapsed();

    let summary = results
        .iter()
        .enumerate()
        .map(|(idx, result)| -> String { format!("Task {idx:>8} -> {:}", result) })
        .join("\n");

    println!(
        "{}\n{}\n{}\n{}",
        summary,
        "-".repeat(72),
        format_args!("Overall   -> {}", overall),
        format_args!(
            "{}: {:?}",
            match num_tasks {
                1 => "Sequential Time",
                _ => "Parallel Time",
            },
            duration
        )
    );
}
