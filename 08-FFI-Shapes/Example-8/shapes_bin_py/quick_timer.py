import sys
import time
from dataclasses import dataclass, field
from typing import Self


@dataclass
class QuickAndDirtyTimer:
    name: str
    start_time: int = field(init=False)

    def __enter__(self) -> Self:
        self.start_time = time.perf_counter_ns()

        return self

    def __exit__(self, exc_type, exc_val, exc_tb) -> Self:
        end_time = time.perf_counter_ns()
        elapsed_time = end_time - self.start_time

        print(f"{self.name:<20}: {elapsed_time:>14,} ns", file=sys.stderr)
