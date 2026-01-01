#!/usr/bin/env python3
"""Benchmark arrayops vs NumPy."""

import time
import array
import sys

try:
    import arrayops as ao
except ImportError:
    print("Error: arrayops not installed. Run 'maturin develop' first.")
    sys.exit(1)

try:
    import numpy as np
except ImportError:
    print("Error: NumPy not installed. Install with 'pip install numpy'")
    sys.exit(1)


def benchmark_sum(size):
    """Benchmark sum operation."""
    # Use smaller values to avoid overflow
    values = [i % 1000 for i in range(size)]

    # NumPy
    arr_np = np.array(values, dtype=np.int32)
    start = time.perf_counter()
    result = np.sum(arr_np)
    numpy_time = (time.perf_counter() - start) * 1000

    # Array + arrayops
    arr_ao = array.array("i", values)
    start = time.perf_counter()
    result = ao.sum(arr_ao)
    arrayops_time = (time.perf_counter() - start) * 1000

    return numpy_time, arrayops_time, result


def benchmark_scale(size):
    """Benchmark scale/multiply operation."""
    values = [i % 1000 for i in range(size)]

    # NumPy vectorized
    arr_np = np.array(values, dtype=np.int32)
    start = time.perf_counter()
    result = arr_np * 2
    numpy_time = (time.perf_counter() - start) * 1000

    # Array + arrayops scale
    arr_ao = array.array("i", values)
    start = time.perf_counter()
    ao.scale(arr_ao, 2.0)
    arrayops_time = (time.perf_counter() - start) * 1000

    return numpy_time, arrayops_time


def benchmark_map(size):
    """Benchmark map operation with Python callable."""
    values = [i % 1000 for i in range(size)]

    # NumPy with vectorize (closest equivalent)
    arr_np = np.array(values, dtype=np.int32)
    func = np.vectorize(lambda x: x * 2)
    start = time.perf_counter()
    result = func(arr_np)
    numpy_time = (time.perf_counter() - start) * 1000

    # Array + arrayops map
    arr_ao = array.array("i", values)
    start = time.perf_counter()
    result = ao.map(arr_ao, lambda x: x * 2)
    arrayops_time = (time.perf_counter() - start) * 1000

    return numpy_time, arrayops_time


def benchmark_filter(size):
    """Benchmark filter operation."""
    values = [i % 1000 for i in range(size)]

    # NumPy boolean indexing
    arr_np = np.array(values, dtype=np.int32)
    start = time.perf_counter()
    result = arr_np[arr_np % 2 == 0]
    numpy_time = (time.perf_counter() - start) * 1000

    # Array + arrayops filter
    arr_ao = array.array("i", values)
    start = time.perf_counter()
    result = ao.filter(arr_ao, lambda x: x % 2 == 0)
    arrayops_time = (time.perf_counter() - start) * 1000

    return numpy_time, arrayops_time


def benchmark_mean(size):
    """Benchmark mean operation."""
    values = [i % 1000 for i in range(size)]

    # NumPy
    arr_np = np.array(values, dtype=np.int32)
    start = time.perf_counter()
    result = np.mean(arr_np)
    numpy_time = (time.perf_counter() - start) * 1000

    # Array + arrayops
    arr_ao = array.array("i", values)
    start = time.perf_counter()
    result = ao.mean(arr_ao)
    arrayops_time = (time.perf_counter() - start) * 1000

    return numpy_time, arrayops_time


def benchmark_std(size):
    """Benchmark standard deviation operation."""
    values = [i % 1000 for i in range(size)]

    # NumPy
    arr_np = np.array(values, dtype=np.int32)
    start = time.perf_counter()
    result = np.std(arr_np)
    numpy_time = (time.perf_counter() - start) * 1000

    # Array + arrayops
    arr_ao = array.array("i", values)
    start = time.perf_counter()
    result = ao.std(arr_ao)
    arrayops_time = (time.perf_counter() - start) * 1000

    return numpy_time, arrayops_time


def get_memory_size(obj):
    """Get memory size of object."""
    import sys

    if isinstance(obj, np.ndarray):
        return obj.nbytes + sys.getsizeof(obj)
    return sys.getsizeof(obj)


def run_benchmarks():
    """Run all benchmarks."""
    sizes = [1000, 100_000, 1_000_000]

    print("=" * 80)
    print("BENCHMARK: arrayops vs NumPy")
    print("=" * 80)
    print()

    for size in sizes:
        print(f"\nArray Size: {size:,} elements")
        print("-" * 80)

        # Sum
        numpy_time, arrayops_time, _ = benchmark_sum(size)
        ratio = numpy_time / arrayops_time if arrayops_time > 0 else 0
        print("Sum:")
        print(f"  NumPy:    {numpy_time:8.3f} ms")
        print(f"  arrayops: {arrayops_time:8.3f} ms")
        print(f"  Ratio:    {ratio:6.2f}x")

        # Scale
        numpy_time, arrayops_time = benchmark_scale(size)
        ratio = numpy_time / arrayops_time if arrayops_time > 0 else 0
        print("Scale (multiply by 2):")
        print(f"  NumPy:    {numpy_time:8.3f} ms")
        print(f"  arrayops: {arrayops_time:8.3f} ms")
        print(f"  Ratio:    {ratio:6.2f}x")

        # Map
        numpy_time, arrayops_time = benchmark_map(size)
        ratio = arrayops_time / numpy_time if numpy_time > 0 else 0
        print("Map (x * 2) with Python callable:")
        print(f"  NumPy:    {numpy_time:8.3f} ms")
        print(f"  arrayops: {arrayops_time:8.3f} ms")
        print(f"  Ratio:    {ratio:6.2f}x (arrayops/NumPy)")

        # Filter
        numpy_time, arrayops_time = benchmark_filter(size)
        ratio = numpy_time / arrayops_time if arrayops_time > 0 else 0
        print("Filter (even numbers):")
        print(f"  NumPy:    {numpy_time:8.3f} ms")
        print(f"  arrayops: {arrayops_time:8.3f} ms")
        print(f"  Ratio:    {ratio:6.2f}x")

        # Mean
        numpy_time, arrayops_time = benchmark_mean(size)
        ratio = numpy_time / arrayops_time if arrayops_time > 0 else 0
        print("Mean:")
        print(f"  NumPy:    {numpy_time:8.3f} ms")
        print(f"  arrayops: {arrayops_time:8.3f} ms")
        print(f"  Ratio:    {ratio:6.2f}x")

        # Std
        numpy_time, arrayops_time = benchmark_std(size)
        ratio = numpy_time / arrayops_time if arrayops_time > 0 else 0
        print("Standard Deviation:")
        print(f"  NumPy:    {numpy_time:8.3f} ms")
        print(f"  arrayops: {arrayops_time:8.3f} ms")
        print(f"  Ratio:    {ratio:6.2f}x")

    # Memory comparison
    print("\n" + "=" * 80)
    print("MEMORY COMPARISON")
    print("=" * 80)
    print()

    for size in [1_000_000]:
        arr_np = np.array(range(size), dtype=np.int32)
        arr_ao = array.array("i", range(size))

        numpy_size = get_memory_size(arr_np)
        array_size = get_memory_size(arr_ao)
        diff = ((numpy_size - array_size) / array_size) * 100

        print(f"Array Size: {size:,} elements (int32)")
        print(f"  NumPy:    {numpy_size / (1024 * 1024):8.2f} MB")
        print(f"  array:    {array_size / (1024 * 1024):8.2f} MB")
        print(f"  Diff:     {diff:6.2f}% (NumPy overhead)")


if __name__ == "__main__":
    run_benchmarks()
