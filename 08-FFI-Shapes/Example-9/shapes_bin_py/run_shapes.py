import logging
import sys
from typing import Final, Generator, Optional, TextIO

from headings import BorderHeading, MultiLineBorderHeading
from quick_timer import QuickAndDirtyTimer
from shapes.shape import Shape
from shapes_py.collection import CompareBy, ShapeCollection

from shapes_py import ShapeFactory, ShapeParser

PROGRAM_HEADING = MultiLineBorderHeading(
    content=(
        "Objects & Inheritance: 2-D Shapes",
        "Thomas J. Kennedy",
    ),
    width=80,
    symbol="-",
)

FACTORY_DESCRIPTION: Final[str] = "\n".join(
    (
        "~" * 38,
        "Available Shapes".center(38),
        "~" * 38,
        ShapeFactory.list_known(),
        "-" * 38,
        f"{ShapeFactory.number_known():>2} shapes available.\n",
    )
)


def main() -> None:
    """
    The main function. In practice I could name this
    anything. The name main was selected purely
    out of familiarity.

    The "if __name__" line below determines what runs
    """

    if len(sys.argv) < 2:
        print("No input file provided.")
        print(f"Usage: {sys.argv[0]:} input_file")
        sys.exit(1)

    shapes_filename = sys.argv[1]

    print(PROGRAM_HEADING)
    print(FACTORY_DESCRIPTION)

    with QuickAndDirtyTimer("Parsing") as _timer:
        shapes = ShapeCollection.read_from_file(shapes_filename)

    if not shapes:
        raise RuntimeError(f"{shapes_filename!r} did not contain any valid shapes")

    print(BorderHeading("Display All Shapes"))
    # Cannot do for loop without an iterator...
    """
    for shp in shapes:
        print(shp)
        print()
    """
    with QuickAndDirtyTimer("Display Shapes") as _timer:
        #  print(shapes)
        #  print()
        shapes.print()

    with QuickAndDirtyTimer("Max Area / Min Perim") as _timer:
        #  smallest_shape = shapes.min(CompareBy.Perimeter)
        #  largest_shape = shapes.max(CompareBy.Area)
        (smallest_shape, largest_shape) = shapes.minmax(CompareBy.Perimeter, CompareBy.Area)

    print(BorderHeading("Display Largest Shape (Area)"))
    print(largest_shape)
    print()

    print(BorderHeading("Display Smallest Shape (Perimeter)"))
    print(smallest_shape)
    print()

    print(BorderHeading("Display Shapes Sorted by Name"))
    #  for shp in sorted(shapes, key=lambda shape: shape.name):
    #  print(shp)
    #  print()
    with QuickAndDirtyTimer("Sort by Name") as _timer:
        shapes.sort(CompareBy.Name)

    with QuickAndDirtyTimer("Display Names") as _timer:
        # Should really just be names
        #  print(shapes.to_name_string())
        shapes.print_names()

    with QuickAndDirtyTimer("Timer Overhead") as _timer:
        with QuickAndDirtyTimer("Inner Test Timer") as _:
            pass


def set_up_logging(level: int = logging.WARN) -> None:
    logger = logging.getLogger("shapes_py")
    logger.setLevel(level)

    handler = logging.StreamHandler(sys.stderr)

    handler.setFormatter(logging.Formatter("%(name)s - %(levelname)s - %(message)s"))

    logger.addHandler(handler)


if __name__ == "__main__":
    try:
        set_up_logging(level=logging.ERROR)
        main()
    except FileNotFoundError as err:
        print(err)
