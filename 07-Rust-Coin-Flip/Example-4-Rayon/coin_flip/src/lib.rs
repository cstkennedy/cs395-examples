pub mod flip_task;

use std::thread;

use rayon::prelude::*;

use flip_task::FlipTask;

pub fn get_num_flips_per_thread(num_threads: usize, num_flips: u64) -> Vec<u64> {
    let num_flips_per_thread = num_flips / (num_threads as u64);
    let num_flips_truncated = num_flips - (num_flips_per_thread * (num_threads as u64));
    let num_flips_for_last = num_flips_per_thread + num_flips_truncated;

    let mut num_flips = vec![num_flips_per_thread; num_threads - 1];
    num_flips.push(num_flips_for_last);

    num_flips
}

pub fn run_parallel(num_threads: usize, num_flips: u64) -> (FlipTask, Vec<FlipTask>) {
    let num_flips = get_num_flips_per_thread(num_threads, num_flips);

    let handles: Vec<_> = num_flips
        .into_par_iter()
        .map(|num_flips_for_thread| {
            FlipTask::simulate_flips(num_flips_for_thread)
        })
        .collect();

    let indiv_results: Vec<FlipTask> = handles;

    let merged_results = indiv_results.par_iter().copied()
        .reduce(FlipTask::default, |merged, result| merged + result);

    (merged_results, indiv_results)
}
