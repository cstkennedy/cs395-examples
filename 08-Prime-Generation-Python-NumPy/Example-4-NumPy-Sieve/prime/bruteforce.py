from __future__ import annotations

from itertools import takewhile
import math
import numpy as np
from typing import Generator


def generate_primes(to_generate: int, sieve_size: int = 10_000_000) -> Generator[int, None, None]:
    """
    Generate a sequence of prime numbers

    Keyword arguments:
        to_generate -- number of primes to generate
    """

    # A woefully incomplete check... but... it is better than nothing
    if to_generate > sieve_size:
        raise ValueError("Sieve must be larger than the number of desired primes")

    # Seed the first two primes
    known_primes = [2, 3]
    yield from known_primes

    # Rely on the fact the 1 is implicitly converted to True and 0 to False
    the_sieve = np.ones(sieve_size, dtype=np.bool_)

    # We know that one is not prime (it is a special case).
    the_sieve[0] = False

    starting_prime_idx = 0
    idx_last_output = 0

    while len(known_primes) < to_generate:
        last_known_prime = known_primes[-1]
        # If the sieve has been exhausted
        # 1.1 is a magic number... chosen with trial and error
        if (1.1 * last_known_prime) >= len(the_sieve):
            break

        upper_bound = last_known_prime ** 2

        for prime in known_primes:
            sieve_start_idx = prime - 1
            the_sieve[sieve_start_idx::prime] = False

        for idx, is_prime in enumerate(the_sieve[starting_prime_idx:upper_bound]):
            if is_prime:
                new_prime = idx + starting_prime_idx + 1
                known_primes.append(new_prime)
                yield new_prime

        starting_prime_idx += 1

        idx_last_output = len(known_primes) - 1




