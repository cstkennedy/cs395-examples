use std::env;

use prime_generation::generate_primes;


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
