import copy

import pytest
from hamcrest import *

from shapes.shape import Shape
from shapes.utils import random_file
from shapes_py import Shape, ShapeParser
from shapes_py.collection import CompareBy, ShapeCollection


@pytest.mark.parametrize(
    "attribute",
    [
        pytest.param(CompareBy.Name, id="Name"),
        pytest.param(CompareBy.Area, id="Area"),
        pytest.param(CompareBy.Perimeter, id="Perimeter"),
    ],
)
@pytest.mark.parametrize("num_lines", [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000])
def bench_shape_collection_sort_shapes(benchmark, attribute: CompareBy, num_lines: int):
    random_shape_filename = random_file(num_lines)
    shapes = ShapeCollection.read_from_file(random_shape_filename)

    benchmark(shapes.sort, attribute)


@pytest.mark.parametrize(
    "attribute",
    [
        pytest.param(CompareBy.Name, id="Name"),
        pytest.param(CompareBy.Area, id="Area"),
        pytest.param(CompareBy.Perimeter, id="Perimeter"),
    ],
)
@pytest.mark.parametrize("num_lines", [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000])
def bench_shape_collection_min_shape(benchmark, attribute: CompareBy, num_lines: int):
    random_shape_filename = random_file(num_lines)
    shapes = ShapeCollection.read_from_file(random_shape_filename)

    benchmark(shapes.min, attribute)


@pytest.mark.parametrize(
    "attribute",
    [
        pytest.param(CompareBy.Name, id="Name"),
        pytest.param(CompareBy.Area, id="Area"),
        pytest.param(CompareBy.Perimeter, id="Perimeter"),
    ],
)
@pytest.mark.parametrize("num_lines", [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000])
def bench_shape_collection_max_shape(benchmark, attribute: CompareBy, num_lines: int):
    random_shape_filename = random_file(num_lines)
    shapes = ShapeCollection.read_from_file(random_shape_filename)

    benchmark(shapes.max, attribute)


def max_wrapper(shapes: list[Shape]):
    return max(shapes, key=lambda shp: shape.name)


@pytest.mark.parametrize("num_lines", [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000])
def bench_shape_collection_max_shape_name(
    benchmark, attribute: CompareBy, num_lines: int
):
    random_shape_filename = random_file(num_lines)
    shapes = ShapeParser.read_shapes(random_shape_filename)

    benchmark(max_wrapper, shapes)
