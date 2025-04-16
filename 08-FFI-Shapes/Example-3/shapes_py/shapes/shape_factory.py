"""
This module provides factory utilities for creating shapes. This includes
recording which Shape types are available.

"""

from __future__ import annotations

import copy
from typing import Optional

from shapes.shape import Shape
from shapes_lib_py import Circle, Square, Triangle, EquilateralTriangle, RightTriangle

import shapes_lib_py

_KNOWN_SHAPES = {
    "Triangle": Triangle,
    "Right Triangle": RightTriangle,
    "Equilateral Triangle": EquilateralTriangle,
    "Square": Square,
    "Circle": Circle
}  # _Dictionary_ of known shapes


def create(name: str) -> Optional[Shape]:
    """
    Create a Shape

    Args:
        name: the shape to be created

    Returns:
        A shape with the specified name or null if no matching shape is found
    """
    
    return shapes_lib_py.ShapeFactory.create(name)


def create_from_dimensions(name: str, values: list[float]) -> Optional[Shape]:
    """
    Create a Shape

    Args:
        name: the shape to be created

        values: dictionary of values corresponding to the data needed
            to inialize a shape

    Returns:
        A shape with the specified name or null if no matching shape is found
    """

    return shapes_lib_py.ShapeFactory.create_with(name, values)



def is_known(name: str) -> bool:
    """
    Determine whether a given shape is known

    Args:
        name: the shape for which to query
    """

    return shapes_lib_py.ShapeFactory.is_known(name)


def list_known() -> str:
    """
    Print a list of known Shapes
    """
    return shapes_lib_py.ShapeFactory.list_known()


def number_known() -> int:
    """
    Determine the number of known Shapes
    """

    return shapes_lib_py.ShapeFactory.number_known()
