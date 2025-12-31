#!/usr/bin/env python3
"""Quick benchmark comparing arrayops vs NumPy performance."""

import time
import array
import arrayops as ao

try:
    import numpy as np

    NUMPY_AVAILABLE = True
except ImportError:
    NUMPY_AVAILABLE = False
    print("NumPy not available, skipping NumPy comparisons")
    print()


def benchmark(func, *args, iterations=100, warmup=10):
    """Run a function multiple times and return average time in seconds."""
    # Warmup
    for _ in range(warmup):
        func(*args)

    times = []
    for _ in range(iterations):
        start = time.perf_counter()
        func(*args)
        end = time.perf_counter()
        times.append(end - start)

    return sum(times) / len(times)


def format_speedup(arrayops_time, numpy_time):
    """Format speedup comparison."""
    if arrayops_time < numpy_time:
        speedup = numpy_time / arrayops_time
        return f"{speedup:.2f}x faster"
    else:
        slowdown = arrayops_time / numpy_time
        return f"{slowdown:.2f}x slower"


def main():
    print("=" * 60)
    print("arrayops vs NumPy Performance Comparison")
    print("=" * 60)
    print()

    if not NUMPY_AVAILABLE:
        return

    sizes = [1_000, 10_000, 100_000, 1_000_000]

    # Sum benchmarks
    print("Sum Operation (array.array('i', range(n)))")
    print("-" * 60)
    for size in sizes:
        # Use smaller size to avoid overflow for sum
        if size > 50_000:
            test_size = 50_000
        else:
            test_size = size

        arr = array.array("i", range(test_size))
        np_arr = np.array(range(test_size), dtype=np.int32)

        ao_time = benchmark(ao.sum, arr, iterations=100)
        np_time = benchmark(np.sum, np_arr, iterations=100)

        speedup = format_speedup(ao_time, np_time)
        print(
            f"  Size {test_size:>8,}: arrayops={ao_time * 1000:6.3f}ms, "
            f"numpy={np_time * 1000:6.3f}ms, {speedup}"
        )
    print()

    # Mean benchmarks
    print("Mean Operation (array.array('i', range(n)))")
    print("-" * 60)
    for size in sizes:
        if size > 50_000:
            test_size = 50_000
        else:
            test_size = size

        arr = array.array("i", range(test_size))
        np_arr = np.array(range(test_size), dtype=np.int32)

        ao_time = benchmark(ao.mean, arr, iterations=100)
        np_time = benchmark(np.mean, np_arr, iterations=100)

        speedup = format_speedup(ao_time, np_time)
        print(
            f"  Size {test_size:>8,}: arrayops={ao_time * 1000:6.3f}ms, "
            f"numpy={np_time * 1000:6.3f}ms, {speedup}"
        )
    print()

    # Min/Max benchmarks
    print("Min Operation (array.array('i', range(n)))")
    print("-" * 60)
    for size in sizes:
        arr = array.array("i", range(size))
        np_arr = np.array(range(size), dtype=np.int32)

        ao_time = benchmark(ao.min, arr, iterations=100)
        np_time = benchmark(np.min, np_arr, iterations=100)

        speedup = format_speedup(ao_time, np_time)
        print(
            f"  Size {size:>8,}: arrayops={ao_time * 1000:6.3f}ms, "
            f"numpy={np_time * 1000:6.3f}ms, {speedup}"
        )
    print()

    print("Max Operation (array.array('i', range(n)))")
    print("-" * 60)
    for size in sizes:
        arr = array.array("i", range(size))
        np_arr = np.array(range(size), dtype=np.int32)

        ao_time = benchmark(ao.max, arr, iterations=100)
        np_time = benchmark(np.max, np_arr, iterations=100)

        speedup = format_speedup(ao_time, np_time)
        print(
            f"  Size {size:>8,}: arrayops={ao_time * 1000:6.3f}ms, "
            f"numpy={np_time * 1000:6.3f}ms, {speedup}"
        )
    print()

    # Add benchmarks
    print("Add Operation (element-wise)")
    print("-" * 60)
    for size in sizes:
        arr1 = array.array("i", range(size))
        arr2 = array.array("i", range(1, size + 1))
        np1 = np.array(range(size), dtype=np.int32)
        np2 = np.array(range(1, size + 1), dtype=np.int32)

        ao_time = benchmark(ao.add, arr1, arr2, iterations=50)
        np_time = benchmark(lambda a, b: a + b, np1, np2, iterations=50)

        speedup = format_speedup(ao_time, np_time)
        print(
            f"  Size {size:>8,}: arrayops={ao_time * 1000:6.3f}ms, "
            f"numpy={np_time * 1000:6.3f}ms, {speedup}"
        )
    print()

    # Sort benchmarks (in-place, need to clone)
    print("Sort Operation (reverse-sorted array, worst case)")
    print("-" * 60)
    for size in [1_000, 10_000, 100_000]:
        values = list(range(size, 0, -1))
        arr = array.array("i", values)
        np_arr = np.array(values, dtype=np.int32)

        def ao_sort():
            arr_copy = array.array("i", arr)
            ao.sort(arr_copy)

        def np_sort():
            arr_copy = np_arr.copy()
            arr_copy.sort()

        ao_time = benchmark(ao_sort, iterations=20)
        np_time = benchmark(np_sort, iterations=20)

        speedup = format_speedup(ao_time, np_time)
        print(
            f"  Size {size:>8,}: arrayops={ao_time * 1000:6.3f}ms, "
            f"numpy={np_time * 1000:6.3f}ms, {speedup}"
        )
    print()

    print("=" * 60)
    print("Note: Results may vary based on hardware, Python version,")
    print("      and whether arrayops was built with 'parallel' feature")
    print("=" * 60)


if __name__ == "__main__":
    main()
