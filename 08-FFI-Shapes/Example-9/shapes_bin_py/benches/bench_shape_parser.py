import copy

import pytest
from hamcrest import *
from shapes.shape import Shape
from shapes.utils import random_file

from shapes_py import ShapeParser


@pytest.mark.parametrize(
    "line",
    [
        "Triangle; 1 2 3",
        "Right Triangle; 3 4",
        "Equilateral Triangle; 5",
        "Square; 5",
        "Circle; 5",
    ],
)
def bench_parse_shape_line_ok(benchmark, line: str):
    benchmark(ShapeParser.read_shape, line)


@pytest.mark.parametrize(
    "line",
    [
        "1337 Haxor; invalid input",
        "1337 Haxor; 1 2 3",
    ],
)
def bench_parse_shape_line_err(benchmark, line: str):
    def wrapped():
        try:
            ShapeParser.read_shape(line)
        except (KeyError, ValueError):
            pass

    benchmark(wrapped)


@pytest.mark.parametrize("num_lines", [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000])
def bench_parse_shape_file(benchmark, num_lines: int):
    random_shape_filename = random_file(num_lines)

    benchmark(ShapeParser.read_shapes, random_shape_filename)
