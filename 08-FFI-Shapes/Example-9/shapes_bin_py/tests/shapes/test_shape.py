import copy

import pytest
from hamcrest import *
from shapes_py.exception import ShapeConversionError

from shapes_py import (
    Circle,
    EquilateralTriangle,
    RightTriangle,
    Shape,
    ShapeFactory,
    Square,
    Triangle,
)


def test_type_mismatch_exception():

    a_shape = ShapeFactory.create("Circle")

    with pytest.raises(ShapeConversionError):
        a_shape.trigger_conversion_error()


@pytest.mark.parametrize(
    "the_cls, the_method",
    [
        (Circle, Shape.try_as_circle),
        (Square, Shape.try_as_square),
        (Triangle, Shape.try_as_triangle),
        (EquilateralTriangle, Shape.try_as_equilateral_triangle),
        (RightTriangle, Shape.try_as_right_triangle),
    ],
)
def test_try_as_ok(the_cls, the_method):
    ref_shape = the_cls()
    name = ref_shape.name

    wrapped_shape = ShapeFactory.create(name)

    unwrapped_shape = the_method(wrapped_shape)
    assert_that(unwrapped_shape, is_(instance_of(the_cls)))


@pytest.fixture
def all_classes_and_try_methods():
    yield [
        (Circle, Shape.try_as_circle),
        (Square, Shape.try_as_square),
        (Triangle, Shape.try_as_triangle),
        (EquilateralTriangle, Shape.try_as_equilateral_triangle),
        (RightTriangle, Shape.try_as_right_triangle),
    ]


@pytest.mark.parametrize(
    "the_cls, the_method",
    [
        (Circle, Shape.try_as_circle),
        (Square, Shape.try_as_square),
        (Triangle, Shape.try_as_triangle),
        (EquilateralTriangle, Shape.try_as_equilateral_triangle),
        (RightTriangle, Shape.try_as_right_triangle),
    ],
)
def test_try_as_err(the_cls, the_method, all_classes_and_try_methods):
    ref_shape = the_cls()
    name = ref_shape.name

    wrapped_shape = ShapeFactory.create(name)

    for some_cls, some_try_method in all_classes_and_try_methods:
        # only call "wrong" try_as methods
        if the_cls == some_cls:
            continue

        with pytest.raises(ShapeConversionError):
            _ = some_try_method(wrapped_shape)
