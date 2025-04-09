use std::time::{Duration, Instant};

use log;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use ::coin_flip::flip_task::FlipTask;
use ::coin_flip::*;

#[pyclass]
#[derive(Debug)]
pub struct FlipResult {
    task: FlipTask,
}

impl From<FlipTask> for FlipResult {
    fn from(src: FlipTask) -> Self {
        FlipResult { task: src }
    }
}

#[pyclass(eq, eq_int)]
#[derive(Debug, PartialEq)]
pub enum SimulationMode {
    Sequential,
    Parallel,
}

#[pyclass]
#[derive(Debug)]
pub struct FlipSummary {
    mode: SimulationMode,
    duration: Duration,
    results: Vec<FlipResult>,
    overall: FlipResult,
}

#[pymethods]
impl FlipSummary {
    fn __str__(&self) -> PyResult<String> {
        let summary = self
            .results
            .iter()
            .enumerate()
            .map(|(idx, result)| -> String { format!("Worker {idx:>2} -> {:}", result.task) })
            .collect::<Vec<String>>()
            .join("\n");

        Ok(format!(
            "{}\n{}\n{}\n{}",
            summary,
            "-".repeat(72),
            format_args!("Overall   -> {}", self.overall.task),
            format_args!(
                "{}: {:?}",
                match self.mode {
                    SimulationMode::Sequential => "Sequential Time",
                    SimulationMode::Parallel => "Parallel Time",
                },
                self.duration
            )
        ))
    }
}

#[pyfunction]
fn do_flips(num_threads: usize, num_flips: u64) -> FlipSummary {
    if num_threads == 0 {
        log::error!("'num_threads' is zero");
    }
    let start = Instant::now();
    let (overall, results) = if num_threads == 1 {
        log::error!("Sequential simulation started");
        let result = FlipTask::simulate_flips(num_flips);

        (result, vec![result])
    } else {
        log::info!("Parallel simulation started");
        run_parallel(num_threads, num_flips)
    };
    log::info!("Simulation complete");
    let duration = start.elapsed();

    log::info!("Collecting results");
    let summary = FlipSummary {
        mode: SimulationMode::Parallel,
        duration,
        results: results.into_iter().map(FlipResult::from).collect(),
        overall: overall.into(),
    };

    log::debug!("{:#?}", summary);

    summary
}

#[pymodule(name = "coin_flip")]
fn coin_flip_py(module: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add logger
    pyo3_log::init();

    module.add_wrapped(wrap_pyfunction!(do_flips))?;
    Ok(())
}
