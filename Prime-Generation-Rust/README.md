# Overview

This is an example of the runtime impact of using FFI to call Rust code from
Python. There are four (4) Python variants benchmarked.

  1. Pure Python (No Rust)
  2. Python where `can_be_divided_by_any` is replaced with a Rust version.
  3. Python where `generate_next` is replaced with a Rust version.
  4. Python where `generate_primes` is replaced with a Rust version.



# Benchmark 1

```console
hyperfine \
    -L num_primes 10,100,1000,10000 'python3.11 prime_number.py {num_primes} 0' \
    'python3.11 prime_number.py {num_primes} 1' \
    'python3.11 prime_number.py {num_primes} 2' \
    'python3.11 prime_number.py {num_primes} 3' \
    'target/release/prime_generation_rs {num_primes}' \
    'target/debug/prime_generation_rs {num_primes}'  \
    --export-markdown=runtime-comparision-0.md
```


| Command                                    | Mean [ms]      | Min [ms] | Max [ms] | Relative       |
| :---                                       | ---:           | ---:     | ---:     | ---:           |
| `python3.11 prime_number.py 10 0`          | 31.8 ± 0.8     | 30.4     | 34.3     | 6.99 ± 0.45    |
| `python3.11 prime_number.py 10 1`          | 32.1 ± 1.2     | 30.8     | 40.0     | 7.04 ± 0.50    |
| `python3.11 prime_number.py 10 2`          | 32.1 ± 0.8     | 31.0     | 35.0     | 7.05 ± 0.46    |
| `python3.11 prime_number.py 10 3`          | 32.0 ± 0.9     | 30.9     | 34.8     | 7.04 ± 0.47    |
| `target/release/prime_generation_rs 10`    | 4.6 ± 0.3      | 4.1      | 6.6      | 1.00           |
| `target/debug/prime_generation_rs 10`      | 4.6 ± 0.3      | 4.2      | 7.4      | 1.01 ± 0.10    |
| `python3.11 prime_number.py 100 0`         | 32.1 ± 0.7     | 31.0     | 34.4     | 7.05 ± 0.45    |
| `python3.11 prime_number.py 100 1`         | 32.2 ± 0.9     | 30.9     | 36.7     | 7.08 ± 0.47    |
| `python3.11 prime_number.py 100 2`         | 32.4 ± 1.1     | 30.9     | 35.5     | 7.11 ± 0.49    |
| `python3.11 prime_number.py 100 3`         | 31.9 ± 0.9     | 30.7     | 34.9     | 7.00 ± 0.47    |
| `target/release/prime_generation_rs 100`   | 4.7 ± 0.3      | 4.3      | 6.5      | 1.02 ± 0.09    |
| `target/debug/prime_generation_rs 100`     | 4.8 ± 0.4      | 4.3      | 7.6      | 1.05 ± 0.10    |
| `python3.11 prime_number.py 1000 0`        | 36.1 ± 0.9     | 34.8     | 38.8     | 7.92 ± 0.52    |
| `python3.11 prime_number.py 1000 1`        | 48.9 ± 1.3     | 47.3     | 51.7     | 10.75 ± 0.71   |
| `python3.11 prime_number.py 1000 2`        | 36.2 ± 0.8     | 35.0     | 38.6     | 7.95 ± 0.51    |
| `python3.11 prime_number.py 1000 3`        | 32.6 ± 0.7     | 31.6     | 35.2     | 7.16 ± 0.46    |
| `target/release/prime_generation_rs 1000`  | 6.0 ± 0.3      | 5.6      | 7.7      | 1.31 ± 0.10    |
| `target/debug/prime_generation_rs 1000`    | 8.9 ± 0.5      | 8.4      | 11.6     | 1.95 ± 0.16    |
| `python3.11 prime_number.py 10000 0`       | 118.6 ± 2.7    | 113.1    | 128.3    | 26.06 ± 1.68   |
| `python3.11 prime_number.py 10000 1`       | 2045.1 ± 121.7 | 1948.0   | 2329.0   | 449.29 ± 38.13 |
| `python3.11 prime_number.py 10000 2`       | 403.1 ± 10.9   | 384.0    | 421.6    | 88.57 ± 5.87   |
| `python3.11 prime_number.py 10000 3`       | 40.0 ± 0.9     | 38.7     | 42.5     | 8.79 ± 0.57    |
| `target/release/prime_generation_rs 10000` | 23.0 ± 0.4     | 22.3     | 25.0     | 5.06 ± 0.32    |
| `target/debug/prime_generation_rs 10000`   | 115.1 ± 1.1    | 112.6    | 116.5    | 25.28 ± 1.55   |

# Benchmark 2

```console
hyperfine \
    -L num_primes 100000 'python3.11 prime_number.py {num_primes} 0' \
    'python3.11 prime_number.py {num_primes} 2' \
    'python3.11 prime_number.py {num_primes} 3' \
    'target/release/prime_generation_rs {num_primes}' \
    'target/debug/prime_generation_rs {num_primes}'  \
    --export-markdown=runtime-comparision-1.md
```


| Command                                     | Mean [s]       | Min [s] | Max [s] | Relative      |
| :---                                        | ---:           | ---:    | ---:    | ---:          |
| `python3.11 prime_number.py 100000 0`       | 1.905 ± 0.043  | 1.855   | 1.955   | 7.48 ± 0.19   |
| `python3.11 prime_number.py 100000 2`       | 35.089 ± 0.950 | 33.560  | 36.853  | 137.78 ± 4.09 |
| `python3.11 prime_number.py 100000 3`       | 0.255 ± 0.003  | 0.248   | 0.258   | 1.00          |
| `target/release/prime_generation_rs 100000` | 0.328 ± 0.003  | 0.319   | 0.331   | 1.29 ± 0.02   |
| `target/debug/prime_generation_rs 100000`   | 3.307 ± 0.054  | 3.250   | 3.397   | 12.99 ± 0.26  |

# Benchmark 3

```console
hyperfine \
    -L num_primes 1000000 \
    'python3.11 prime_number.py {num_primes} 3' \
    'target/release/prime_generation_rs {num_primes}' \
    --export-markdown=runtime-comparision-2.md
```


| Command                                      | Mean [s]      | Min [s] | Max [s] | Relative    |
| :---                                         | ---:          | ---:    | ---:    | ---:        |
| `python3.11 prime_number.py 1000000 3`       | 6.657 ± 0.095 | 6.554   | 6.824   | 1.00        |
| `target/release/prime_generation_rs 1000000` | 7.652 ± 0.093 | 7.537   | 7.764   | 1.15 ± 0.02 |
