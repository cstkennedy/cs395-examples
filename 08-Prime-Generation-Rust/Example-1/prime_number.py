#! /usr/bin/env python3

# Programmer : Thomas J. Kennedy

import sys
from enum import Enum

import prime as python_prime_generation
import rust_prime_generation


class ExecMode(Enum):
    PURE_PYTHON = 0
    RUST_DIVISION_CHECK = 1
    RUST_COMPUTE_NEXT = 2
    PURE_RUST = 3


PROGRAM_HEADING = "\n".join(
    (
        "-" * 80,
        "Prime Number Generation".center(80),
        "Thomas J. Kennedy".center(80),
        "-" * 80,
    )
)


def __parse_args() -> (int, ExecMode):
    """
    Parse command line arguments (num_primes). Default to 10.
    """

    try:
        num_primes = int(sys.argv[1])

    except (IndexError, ValueError) as _err:
        num_primes = 10

    try:
        mode = ExecMode(int(sys.argv[2]))

    except IndexError as _err:
        mode = ExecMode(0)

    return num_primes, mode


def main():
    """
    The main function. In practice I could name this
    anything. The name main was selected purely
    out of familiarity.

    The "if __name__" line below determines what runs

    """

    print(PROGRAM_HEADING)

    num_primes, mode = __parse_args()

    match mode:
        case ExecMode.PURE_PYTHON:
            primes = python_prime_generation.bruteforce.generate_primes(num_primes)

        case ExecMode.RUST_DIVISION_CHECK:
            primes = python_prime_generation.bruteforce_ffi_1.generate_primes(num_primes)

        case ExecMode.RUST_COMPUTE_NEXT:
            primes = python_prime_generation.bruteforce_ffi_2.generate_primes(num_primes)

        case ExecMode.PURE_RUST:
            primes = rust_prime_generation.generate_primes(num_primes)

        case _:
            raise NotImplementedError("Mode is not yet supported.")

    for prime in primes:
        print(prime)


if __name__ == "__main__":
    main()
