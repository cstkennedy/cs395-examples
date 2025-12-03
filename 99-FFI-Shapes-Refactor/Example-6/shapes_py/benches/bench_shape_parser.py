import copy
import random

import pytest
from hamcrest import *

from shapes.shape import Shape
from shapes_lib_py import ShapeParser


@pytest.mark.parametrize(
    "line",
    [
        "Triangle; 1 2 3",
        "Right Triangle; 3 4",
        "Equilateral Triangle; 5",
        "Square; 5",
        "Circle; 5",
    ]
)
def bench_parse_shape_line(benchmark, line: str):
    benchmark(ShapeParser.read_shape, line)

@pytest.mark.parametrize(
    "line",
    [
        "1337 Haxor; invalid input",
    ]
)
def bench_parse_shape_line_failure(benchmark, line: str):
    def wrapped():
        try:
            ShapeParser.read_shape(line)
        except KeyError:
            pass

    benchmark(wrapped)


@pytest.mark.parametrize("num_lines", [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000])
def bench_parse_shape_file(benchmark, num_lines: int):
    KNOWN_SHAPES = [
        ("Triangle", 3),
        ("Right Triangle", 2),
        ("Equilateral Triangle", 1),
        ("Square", 1),
        ("Circle", 1)
    ]

    INVALID_SHAPE_RATE = 0.10
    INVALID_DIM_RATE = 0.10

    num_valid_shapes = 0

    random_shape_filename = f"random-shapes-{num_lines}.txt"
    with open(random_shape_filename, "w") as random_shape_file:
        for _ in range(num_lines):
            # Create an unknown shape"
            if random.random() < INVALID_SHAPE_RATE:
                random_shape_file.write("LOL Not a Shape; 1 2 3 4 5\n")
                continue

            shape, dim_count = random.choice(KNOWN_SHAPES)

            dims = " ".join((f"{random.uniform(1, 10)}" for _ in range(dim_count)))

            random_shape_file.write(f"{shape}; {dims}\n")

            num_valid_shapes += 1

    benchmark(ShapeParser.read_shapes, random_shape_filename)





