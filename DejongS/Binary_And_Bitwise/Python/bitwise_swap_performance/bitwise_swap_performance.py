#!/usr/bin/env python3

import time

LOOP_COUNT: int = 100_000_000

def do_bitwise_swap(a: int, b: int) -> tuple[int, int]:
    a = a ^ b
    b = a ^ b
    a = a ^ b

    return (a, b)


def do_python_swap(a: int, b: int) -> tuple[int, int]:
    a, b = b, a
    return (a, b)


def do_temp_swap(a: int, b: int) -> tuple[int, int]:
    temp = a
    a = b
    b = temp

    return (a, b)


def main() -> None:
    a: int = 555
    b: int = 7777

    print(do_bitwise_swap(a, b))
    print(do_python_swap(a, b))
    print(do_temp_swap(a, b))

    start: float = time.perf_counter()
    for _ in range(LOOP_COUNT):
        do_bitwise_swap(a, b)

    end: float = time.perf_counter()

    print(f"Bitwise Swap: {end - start}")

    start: float = time.perf_counter()
    for _ in range(LOOP_COUNT):
        do_python_swap(a, b)

    end: float = time.perf_counter()

    print(f"Python Swap: {end - start}")

    start: float = time.perf_counter()
    for _ in range(LOOP_COUNT):
        do_temp_swap(a, b)

    end: float = time.perf_counter()

    print(f"Temp Swap: {end - start}")


if __name__ == "__main__":
  main()

