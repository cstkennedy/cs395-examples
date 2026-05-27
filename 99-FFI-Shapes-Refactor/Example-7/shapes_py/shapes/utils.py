import random

def random_file(num_lines: int) -> str:
    KNOWN_SHAPES = [
        ("Triangle", 3),
        ("Right Triangle", 2),
        ("Equilateral Triangle", 1),
        ("Square", 1),
        ("Circle", 1)
    ]

    INVALID_SHAPE_RATE = 0.10
    INVALID_DIM_RATE = 0.10

    num_valid_shapes = 0

    random_shape_filename = f"random-shapes-{num_lines}.txt"
    with open(random_shape_filename, "w") as random_shape_file:
        for _ in range(num_lines):
            # Create an unknown shape"
            if random.random() < INVALID_SHAPE_RATE:
                random_shape_file.write("LOL Not a Shape; 1 2 3 4 5\n")
                continue

            shape, dim_count = random.choice(KNOWN_SHAPES)

            dims = " ".join((f"{random.uniform(1, 10)}" for _ in range(dim_count)))

            random_shape_file.write(f"{shape}; {dims}\n")

            num_valid_shapes += 1

    return random_shape_filename
