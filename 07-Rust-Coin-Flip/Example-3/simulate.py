import random
import sys

from typing import Final

import coin_flip

DEFAULT_NUM_TRIALS: Final[int] = 10_000
SUMMARY_FMT_STR: Final[str] = "# Heads: {:>6d} ({:>6.4f}) / # Tails {:>6d} ({:>6.4f})"


def main() -> None:
    try:
        num_threads = int(sys.argv[1])

    except (IndexError, ValueError) as _err:
        num_threads = 1

    if num_threads > 32:
        num_threads = 32

    try:
        num_trials = int(sys.argv[2])

    except (IndexError, ValueError) as _err:
        num_trials = DEFAULT_NUM_TRIALS

    results = coin_flip.do_flips(num_threads, num_trials)

    print(results)


if __name__ == "__main__":
    main()
