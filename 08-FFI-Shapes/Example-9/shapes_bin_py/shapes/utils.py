import random
from typing import Final

KNOWN_SHAPES: Final[tuple[str, int]] = [
    ("Triangle", 3),
    ("Right Triangle", 2),
    ("Equilateral Triangle", 1),
    ("Square", 1),
    ("Circle", 1),
]

INVALID_SHAPE_RATE: Final[float] = 0.10
INVALID_DIM_RATE: Final[float] = 0.10


def random_file(num_lines: int) -> str:
    random_shape_filename = f"random-shapes-{num_lines}.txt"

    with open(random_shape_filename, "w") as random_shape_file:
        num_invalid_shapes = 0

        for _ in range(0, num_lines):
            # Create an unknown shape"
            if random.random() < INVALID_SHAPE_RATE:
                num_invalid_shapes += 1

                random_shape_file.write("LOL Not a Shape; 1 2 3 4 5\n")
                continue

            shape, dim_count = random.choice(KNOWN_SHAPES)

            dims = (random.uniform(1, 10) for _ in range(dim_count))
            dims = (f"{dim:.4f}" for dim in dims)
            dims = " ".join(dims)

            random_shape_file.write(f"{shape}; {dims}\n")

        _num_valid_shapes = num_lines - num_invalid_shapes

    return random_shape_filename
