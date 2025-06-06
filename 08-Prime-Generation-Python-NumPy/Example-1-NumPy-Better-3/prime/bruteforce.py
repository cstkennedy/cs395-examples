from __future__ import annotations

from itertools import takewhile
import math
import numpy as np


def __can_be_divided_by_any(known_primes: np.array, next_prime: int):
    """
    Iterate over all known primes and check the next_prime.

    Returns:
        If next_prime can be evenly divided by any previously known prime
        return True. Return False otherwise
    """
    next_prime = np.uint32(next_prime)
    next_sqrt = np.sqrt(next_prime)
    # Grab items up to (and including) the sqrt(next_prime)
    limited = np.extract(known_primes <= next_sqrt, known_primes)
    remainders = next_prime % limited

    return (np.count_nonzero(remainders)) != len(remainders)


def generate_primes(to_generate):
    """
    Generate a sequence of prime numbers

    Keyword arguments:
        to_generate -- number of primes to generate
    """

    known_primes = np.zeros(to_generate, dtype=np.uint32)
    known_primes[:2] = [2, 3]

    for next_prime in known_primes[:2]:
        yield next_prime

    #  for idx in range(3, to_generate + 1):
    for idx in range(2, to_generate):
        # prime from which to start calculations
        next_prime = known_primes[idx - 1]

        # true once a prime number has been identified
        is_prime = False

        # Halt when a prime number has been identified
        while not is_prime:
            # Guess the next prime
            next_prime += 2
            is_prime = not __can_be_divided_by_any(known_primes[:idx], next_prime)
            #  print(f"{next_prime=} {is_prime=}")

        known_primes[idx] = next_prime

        yield next_prime
