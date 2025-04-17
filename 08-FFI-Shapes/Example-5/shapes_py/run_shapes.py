import logging
import sys
from typing import Final, Generator, Optional, TextIO

from headings import BorderHeading, MultiLineBorderHeading
from shapes.shape import Shape
from shapes_lib_py import ShapeFactory, ShapeParser

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


def read_shapes(shapes_in: TextIO) -> Generator[Shape | None, None, None]:
    """
    T.B.W.
    """

    for line in shapes_in:
        try:
            yield ShapeParser.read_shape(line)

        except KeyError as _err:
            logging.warning(f"Skipped {line=!r:} due to unknown shape.")

        except ValueError as _err:
            logging.warning(f"Skipped shape {line=!r:} due to malformed line.")


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

    shapes = ShapeParser.read_shapes(shapes_filename)
    # fmt: off
    #  with open(shapes_filename, "r") as shapes_in:
        #  shapes = [shp for shp in read_shapes(shapes_in) if shp is not None]
    # fmt: on

    print(BorderHeading("Display All Shapes"))
    for shp in shapes:
        print(shp)
        print()

    print(BorderHeading("Display Largest Shape (Area)"))
    largest_shape = max(shapes, key=lambda shape: shape.area())
    print(largest_shape)
    print()

    print(BorderHeading("Display Smallest Shape (Perimeter)"))
    smallest_shape = min(shapes, key=lambda shape: shape.perimeter())
    print(smallest_shape)
    print()

    print(BorderHeading("Display Shapes Sorted by Name"))
    for shp in sorted(shapes, key=lambda shape: shape.name):
        print(shp)
        print()


if __name__ == "__main__":
    try:
        main()
    except FileNotFoundError as err:
        print(err)
