hyperfine -L dir Example-1,Example-1-NumPy,Example-1-NumPy-Better,Example-1-NumPy-Better-2,Example-1-NumPy-Better-3 \
    -L num_primes 10,100,1000,10000 \
    "uv run --directory {dir} prime_number.py {num_primes}"

# uv run --directory Example-1 prime_number.py 10
# uv run --directory Example-1-NumPy prime_number.py 10
# uv run --directory Example-1-NumPy-Better prime_number.py 10
# uv run --directory Example-1-NumPy-Better-2 prime_number.py 10
# uv run --directory Example-1-NumPy-Better-3 prime_number.py 10
# uv run --directory Example-4-NumPy-Sieve prime_number.py 10 100

hyperfine -L dir Example-4-NumPy-Sieve \
    -L num_primes 10,100,1000,10000,100000 \
    "uv run --directory {dir} prime_number.py {num_primes} 1000000"
