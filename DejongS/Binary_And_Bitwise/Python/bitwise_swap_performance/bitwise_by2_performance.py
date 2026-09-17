#!/usr/bin/env python3

import time

LOOP_COUNT: int = 1_000_000

def do_bitwise_by2(a: int, b: int) -> int:
    return a << b


def do_python_by2(a: int, b: int) -> int:
    # a * (2 ** b) is functionaly equivalent to a * pow(2, b)
    return a * (2 ** b)
    # return a * pow(2, b)


def do_loop_by2(a: int, b: int) -> int:
    for _ in range(b):
        a *= 2

    return a


def main() -> None:
    a: int = 555
    b: int = 7777

    print(do_bitwise_by2(10, 4))
    print(do_python_by2(10, 4))
    print(do_loop_by2(10, 4))

    start: float = time.perf_counter()
    for _ in range(LOOP_COUNT):
        do_bitwise_by2(a, b)

    end: float = time.perf_counter()

    print(f"Bitwise By2: {end - start}")

    start: float = time.perf_counter()
    for _ in range(LOOP_COUNT):
        do_python_by2(a, b)

    end: float = time.perf_counter()

    print(f"Python By2: {end - start}")

    start: float = time.perf_counter()
    for _ in range(LOOP_COUNT):
        do_loop_by2(a, b)

    end: float = time.perf_counter()

    print(f"Loop By2: {end - start}")


if __name__ == "__main__":
  main()

