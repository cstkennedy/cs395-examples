use std::env;

/// Iterate over all known primes and check the next_prime.
///
/// Returns:
///     If next_prime can be evenly divided by any previously known prime
///     return True. Return False otherwise
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

/// Generate a sequence of prime numbers
///
/// Keyword arguments:
///     to_generate -- number of primes to generate
fn generate_primes(to_generate: usize) -> Vec<u64>
{
    let mut known_primes = Vec::with_capacity(to_generate);
    known_primes.push(2);
    known_primes.push(3);

    for _idx in 3..=to_generate {
        // prime from which to start calculations
        let mut next_prime = *known_primes.iter().last().unwrap();

        // true once a prime number has been identified
        let mut is_prime = false;

        // Halt when a prime number has been identified
        while !is_prime {
            // Guess the next prime
            next_prime += 2;
            is_prime = !__can_be_divided_by_any(&known_primes, next_prime);
        }

        known_primes.push(next_prime);
    }

    known_primes
}


fn main() {
    let num_primes: usize = env::args()
        .nth(1)
        .unwrap_or("10".into())
        .parse()
        .unwrap_or(10);

    let primes = generate_primes(num_primes);

    for (_idx, prime) in primes.iter().enumerate() {
        println!("{}", prime);
    }
}
