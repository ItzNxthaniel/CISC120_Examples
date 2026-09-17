#!/usr/bin/env python3

import time

LOOP_COUNT: int = 100_000_000

def do_bitwise_mod2(a: int) -> int:
    return a & 1


def do_python_mod2(a: int) -> int:
    return a % 2


def main() -> None:
    a: int = 555

    print(do_bitwise_mod2(10))
    print(do_python_mod2(10))

    start: float = time.perf_counter()
    for _ in range(LOOP_COUNT):
        do_bitwise_mod2(a)

    end: float = time.perf_counter()

    print(f"Bitwise mod2: {end - start}")

    start: float = time.perf_counter()
    for _ in range(LOOP_COUNT):
        do_python_mod2(a)

    end: float = time.perf_counter()

    print(f"Python mod2: {end - start}")


if __name__ == "__main__":
  main()

