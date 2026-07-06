| Command                                       | Mean [ms]       | Min [ms] | Max [ms] | Relative         |
| :---                                          | ---:            | ---:     | ---:     | ---:             |
| `uv run create_shapes_file.py 1`              | 41.1 ± 1.0      | 39.5     | 45.0     | 6.46 ± 0.51      |
| `../target/release/shapes_rand_file 1`        | 6.5 ± 0.5       | 5.6      | 9.0      | 1.01 ± 0.11      |
| `uv run create_shapes_file.py 10`             | 41.1 ± 1.3      | 39.8     | 47.7     | 6.45 ± 0.52      |
| `../target/release/shapes_rand_file 10`       | 6.4 ± 0.5       | 5.6      | 10.4     | 1.00             |
| `uv run create_shapes_file.py 100`            | 42.4 ± 6.8      | 39.8     | 95.2     | 6.66 ± 1.18      |
| `../target/release/shapes_rand_file 100`      | 6.5 ± 0.6       | 5.6      | 11.6     | 1.01 ± 0.12      |
| `uv run create_shapes_file.py 1000`           | 42.4 ± 1.2      | 41.0     | 46.3     | 6.65 ± 0.53      |
| `../target/release/shapes_rand_file 1000`     | 7.2 ± 2.3       | 6.0      | 30.2     | 1.12 ± 0.37      |
| `uv run create_shapes_file.py 10000`          | 55.0 ± 1.6      | 53.2     | 59.9     | 8.64 ± 0.70      |
| `../target/release/shapes_rand_file 10000`    | 10.2 ± 4.6      | 7.8      | 35.9     | 1.61 ± 0.73      |
| `uv run create_shapes_file.py 100000`         | 179.8 ± 3.0     | 172.9    | 185.1    | 28.23 ± 2.17     |
| `../target/release/shapes_rand_file 100000`   | 27.9 ± 1.5      | 26.8     | 37.9     | 4.38 ± 0.40      |
| `uv run create_shapes_file.py 1000000`        | 1424.9 ± 31.8   | 1392.6   | 1478.2   | 223.70 ± 17.52   |
| `../target/release/shapes_rand_file 1000000`  | 216.8 ± 4.6     | 210.8    | 226.5    | 34.04 ± 2.66     |
| `uv run create_shapes_file.py 10000000`       | 13749.0 ± 284.3 | 13462.1  | 14190.2  | 2158.44 ± 168.11 |
| `../target/release/shapes_rand_file 10000000` | 2057.2 ± 24.1   | 2039.9   | 2123.9   | 322.96 ± 24.54   |
