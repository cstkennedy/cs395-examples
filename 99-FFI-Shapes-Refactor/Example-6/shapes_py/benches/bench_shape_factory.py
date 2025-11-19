import copy

import pytest
from hamcrest import *

from shapes.shape import Shape
from shapes_lib_py import ShapeFactory

"""
**This is technically a set of Integration Tests**

1 - Does this piece of code perform the operations
    it was designed to perform?

2 - Does this piece of code do something it was not
    designed to perform?

1 Test per mutator
"""


@pytest.fixture
def known_shape_names_and_number():
    known_names = (
        "Circle",
        "Square",
        "Triangle",
        "Equilateral Triangle",
        "Right Triangle",
    )

    number_known = len(known_names)

    yield known_names, number_known


def bench_create_shape_success(benchmark):
    """
    Create a known valid Shape
    """

    benchmark(ShapeFactory.create, "Circle")


def bench_create_shape_failure(benchmark):
    """
    Try to create a known invalid Shape
    """

    def wrapper():
        try:
            _ = ShapeFactory.create("Lol Nope")

        except KeyError:
            pass

    benchmark(wrapper)


def bench_is_known_success(benchmark):
    """
    Create a known valid Shape
    """

    benchmark(ShapeFactory.is_known, "Circle")


def bench_is_known_failure(benchmark):
    """
    Try to create a known invalid Shape
    """

    benchmark(ShapeFactory.is_known, "Lol Nope")


def bench_list_known(benchmark):
    benchmark(ShapeFactory.list_known)


def bench_number_known(benchmark):
    benchmark(ShapeFactory.number_known)
