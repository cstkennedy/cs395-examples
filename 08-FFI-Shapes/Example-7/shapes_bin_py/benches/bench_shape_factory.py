import copy

import pytest
from hamcrest import *

from shapes.shape import Shape
from shapes_py import ShapeFactory


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


@pytest.mark.parametrize(
    "name", ["Circle", "Square", "Triangle", "Right Triangle", "Equilateral Triangle"]
)
def bench_create(benchmark, name):
    """
    Create a known valid Shape
    """

    benchmark(ShapeFactory.create, name)


def bench_create_invalid_name_1_lambda(benchmark):
    """
    Try to create a known invalid Shape
    """

    def wrapper():
        try:
            _ = ShapeFactory.create("Lol Nope")

        except KeyError:
            pass

    benchmark(wrapper)


def wrapped_create():
    try:
        _ = ShapeFactory.create("Lol Nope")

    except KeyError:
        pass


def bench_create_invalid_name_2_function(benchmark):
    """
    Try to create a known invalid Shape
    """

    benchmark(wrapped_create)


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
