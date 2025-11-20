import copy

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
