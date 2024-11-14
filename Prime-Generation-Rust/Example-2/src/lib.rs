use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, wrap_pymodule};

fn __can_be_divided_by_any(known_primes: &[u64], next_prime: u64) -> bool {

    let sqrt_of_next_prime = (next_prime as f64).sqrt().ceil() as u64;

    let num_non_zero_results: usize = known_primes
        .iter()
        .take_while(|prime| *prime <= &sqrt_of_next_prime)
        .map(|prime| next_prime % prime)
        .filter(|remainder| *remainder == 0)
        .count();

    return num_non_zero_results > 0;
}

/// Iterate over all known primes and check the next_prime.
///
/// Returns:
///     If next_prime can be evenly divided by any previously known prime
///     return True. Return False otherwise
#[pyfunction]
fn can_be_divided_by_any(known_primes: Vec<u64>, next_prime: u64) -> bool {
    __can_be_divided_by_any(&known_primes, next_prime)
}

fn __compute_next(known_primes: &[u64]) -> u64 {
    let mut next_prime = *known_primes.iter().last().unwrap();

    // true once a prime number has been identified
    let mut is_prime = false;

    // Halt when a prime number has been identified
    while !is_prime {
        // Guess the next prime
        next_prime += 2;
        is_prime = !__can_be_divided_by_any(&known_primes, next_prime);
    }

    next_prime
}

#[pyfunction]
fn compute_next(known_primes: Vec<u64>) -> u64 {
    __compute_next(&known_primes)
}

/// Generate a sequence of prime numbers
///
/// Keyword arguments:
///     to_generate -- number of primes to generate
#[pyfunction]
fn generate_primes(to_generate: usize) -> Vec<u64>
{
    let mut known_primes = Vec::with_capacity(to_generate);
    known_primes.push(2);
    known_primes.push(3);

    for _idx in 3..=to_generate {
        let next_prime = __compute_next(&known_primes);

        known_primes.push(next_prime);
    }

    known_primes
}


#[pymodule]
fn rust_prime_generation(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_wrapped(wrap_pyfunction!(can_be_divided_by_any))?;
    module.add_wrapped(wrap_pyfunction!(compute_next))?;
    module.add_wrapped(wrap_pyfunction!(generate_primes))?;
    Ok(())
}
