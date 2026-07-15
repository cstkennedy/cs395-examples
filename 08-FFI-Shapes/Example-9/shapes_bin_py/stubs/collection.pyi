from os import PathLike
from typing import Final, final

from . import Shape

@final
class CompareBy:
    Area: Final[CompareBy]
    Name: Final[CompareBy]
    Perimeter: Final[CompareBy]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class ShapeCollection:
    def __str__(self, /) -> str: ...
    def max(self, /, attribute: CompareBy) -> Shape:
        """
        Get a copy of the "largest" element using a specified attribute.

        # Raises

        ValueError is ShapeCollection is empty
        """

    def min(self, /, attribute: CompareBy) -> Shape:
        """
        Get a copy of the "smallest" element using a specified attribute.

        # Raises

        ValueError is ShapeCollection is empty
        """

    def minmax(self, /, attribute: CompareBy) -> tuple[Shape, Shape]: ...
    @staticmethod
    def read_from_file(filename: str | PathLike[str]) -> ShapeCollection: ...
    def sort(self, /, attribute: CompareBy) -> None:
        """
        Sort the ShapeCollection in place
        """

    def to_name_string(self, /) -> str: ...
