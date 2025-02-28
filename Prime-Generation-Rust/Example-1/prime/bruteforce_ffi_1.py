import math
from itertools import takewhile
from typing import Generator

from rust_prime_generation import can_be_divided_by_any


def compute_next(known_primes: list[int]) -> int:
    candidate_prime = known_primes[-1]

    # true once a prime number has been identified
    is_prime = False

    # Halt when a prime number has been identified
    while not is_prime:
        # Guess the next prime
        candidate_prime += 2
        is_prime = not can_be_divided_by_any(known_primes, candidate_prime)

    return candidate_prime


def generate_primes(to_generate: int) -> Generator[int, None, None]:
    """
    Generate a sequence of prime numbers

    Keyword arguments:
        to_generate -- number of primes to generate
    """

    known_primes = [2, 3]

    for next_prime in known_primes:
        yield next_prime

    for _i in range(3, to_generate + 1):
        next_prime = compute_next(known_primes)
        known_primes.append(next_prime)

        yield next_prime
