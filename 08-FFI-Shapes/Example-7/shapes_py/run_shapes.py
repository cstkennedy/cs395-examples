import logging
import sys
from typing import Final, Generator, Optional, TextIO

from headings import BorderHeading, MultiLineBorderHeading
from shapes.shape import Shape
from shapes_lib_py import ShapeFactory, ShapeParser, ShapeCollection, CompareBy

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

    shapes = ShapeCollection.read_from_file(shapes_filename)

    if not shapes:
        raise RuntimeError(f"{shapes_filename!r} did not contain any valid shapes")

    print(BorderHeading("Display All Shapes"))
    print(shapes)
    print()

    print(BorderHeading("Display Largest Shape (Area)"))
    #  largest_shape = max(shapes, key=lambda shape: shape.area())
    largest_shape = shapes.max(CompareBy.Area)
    print(largest_shape)
    print()

    print(BorderHeading("Display Smallest Shape (Perimeter)"))
    #  smallest_shape = min(shapes, key=lambda shape: shape.perimeter())
    smallest_shape = shapes.min(CompareBy.Perimeter)
    print(smallest_shape)
    print()

    print(BorderHeading("Display Shapes Sorted by Name"))
    #  for shp in sorted(shapes, key=lambda shape: shape.name):
        #  print(shp)
        #  print()
    shapes.sort(CompareBy.Name)
    print(shapes)


def set_up_logging(level: int = logging.WARN) -> None:
    logger = logging.getLogger("shapes_lib_py")
    logger.setLevel(level)

    handler = logging.StreamHandler(sys.stderr)

    handler.setFormatter(logging.Formatter("%(name)s - %(levelname)s - %(message)s"))

    logger.addHandler(handler)


if __name__ == "__main__":
    try:
        set_up_logging(level=logging.INFO)
        main()
    except FileNotFoundError as err:
        print(err)
