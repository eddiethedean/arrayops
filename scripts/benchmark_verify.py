#!/usr/bin/env python3
"""Verify benchmark numbers in comparison documents match actual benchmarks."""

import time
import array
import sys

try:
    import arrayops as ao
except ImportError:
    print("Error: arrayops not installed.")
    sys.exit(1)

try:
    import numpy as np
    NUMPY_AVAILABLE = True
except ImportError:
    NUMPY_AVAILABLE = False

try:
    import polars as pl
    POLARS_AVAILABLE = True
except ImportError:
    POLARS_AVAILABLE = False


def benchmark_func(func, *args, iterations=100, warmup=10):
    """Benchmark a function with warmup and multiple iterations, return min time."""
    # Warmup
    for _ in range(warmup):
        func(*args)
    
    # Benchmark
    times = []
    for _ in range(iterations):
        start = time.perf_counter()
        result = func(*args)
        end = time.perf_counter()
        times.append((end - start) * 1000)  # Convert to ms
    
    return min(times)


def verify_numpy_benchmarks():
    """Verify NumPy benchmark numbers from comparison-numpy.md."""
    if not NUMPY_AVAILABLE:
        print("NumPy not available, skipping NumPy verification")
        return
    
    print("=" * 80)
    print("VERIFYING: comparison-numpy.md benchmarks (1M int32 elements)")
    print("=" * 80)
    print()
    
    size = 1_000_000
    values = [i % 1000 for i in range(size)]  # Modulo to avoid overflow
    
    # Expected values from docs
    expected = {
        "sum": {"numpy": 0.297, "arrayops": 7.519, "ratio": 25.3},
        "scale": {"numpy": 0.300, "arrayops": 4.763, "ratio": 15.9},
        "mean": {"numpy": 0.307, "arrayops": 7.132, "ratio": 23.2},
        "filter": {"numpy": 3.321, "arrayops": 204.230, "ratio": 61.5},
    }
    
    print("Operation | Expected NumPy | Actual NumPy | Expected arrayops | Actual arrayops | Ratio")
    print("-" * 80)
    
    # Sum
    arr_np = np.array(values, dtype=np.int32)
    arr_ao = array.array("i", values)
    numpy_time = benchmark_func(np.sum, arr_np, iterations=100)
    arrayops_time = benchmark_func(ao.sum, arr_ao, iterations=100)
    ratio = numpy_time / arrayops_time if arrayops_time > 0 else 0
    exp = expected["sum"]
    print(f"Sum | {exp['numpy']:.3f} | {numpy_time:.3f} | {exp['arrayops']:.3f} | {arrayops_time:.3f} | {ratio:.1f}x")
    
    # Scale
    arr_np = np.array(values, dtype=np.int32)
    arr_ao = array.array("i", values)
    arr_copy = array.array("i", values)
    def scale_op():
        arr_copy[:] = arr_ao[:]
        ao.scale(arr_copy, 2.0)
    numpy_time = benchmark_func(lambda: arr_np * 2, iterations=100)
    arrayops_time = benchmark_func(scale_op, iterations=100)
    ratio = numpy_time / arrayops_time if arrayops_time > 0 else 0
    exp = expected["scale"]
    print(f"Scale | {exp['numpy']:.3f} | {numpy_time:.3f} | {exp['arrayops']:.3f} | {arrayops_time:.3f} | {ratio:.1f}x")
    
    # Mean
    arr_np = np.array(values, dtype=np.int32)
    arr_ao = array.array("i", values)
    numpy_time = benchmark_func(np.mean, arr_np, iterations=100)
    arrayops_time = benchmark_func(ao.mean, arr_ao, iterations=100)
    ratio = numpy_time / arrayops_time if arrayops_time > 0 else 0
    exp = expected["mean"]
    print(f"Mean | {exp['numpy']:.3f} | {numpy_time:.3f} | {exp['arrayops']:.3f} | {arrayops_time:.3f} | {ratio:.1f}x")
    
    # Filter
    threshold = size // 2
    arr_np = np.array(values, dtype=np.int32)
    arr_ao = array.array("i", values)
    numpy_time = benchmark_func(lambda: arr_np[arr_np > threshold], iterations=50)
    arrayops_time = benchmark_func(lambda: ao.filter(arr_ao, lambda x: x > threshold), iterations=20)
    ratio = numpy_time / arrayops_time if arrayops_time > 0 else 0
    exp = expected["filter"]
    print(f"Filter | {exp['numpy']:.3f} | {numpy_time:.3f} | {exp['arrayops']:.3f} | {arrayops_time:.3f} | {ratio:.1f}x")
    
    print()


def verify_polars_benchmarks():
    """Verify Polars benchmark numbers from comparison-polars.md."""
    if not POLARS_AVAILABLE:
        print("Polars not available, skipping Polars verification")
        return
    
    print("=" * 80)
    print("VERIFYING: comparison-polars.md benchmarks (1M int32 elements)")
    print("=" * 80)
    print()
    
    size = 1_000_000
    values = [i % 1000 for i in range(size)]  # Modulo to avoid overflow
    
    # Expected values from docs
    expected = {
        "sum": {"polars": 0.312, "arrayops": 7.519, "ratio": 24.1},
        "scale": {"polars": 0.425, "arrayops": 4.763, "ratio": 11.2},
        "mean": {"polars": 0.335, "arrayops": 7.132, "ratio": 21.3},
        "filter": {"polars": 3.521, "arrayops": 204.230, "ratio": 58.0},
    }
    
    print("Operation | Expected Polars | Actual Polars | Expected arrayops | Actual arrayops | Ratio")
    print("-" * 80)
    
    # Sum
    series = pl.Series("values", values)
    arr_ao = array.array("i", values)
    polars_time = benchmark_func(lambda: series.sum(), iterations=100)
    arrayops_time = benchmark_func(ao.sum, arr_ao, iterations=100)
    ratio = polars_time / arrayops_time if arrayops_time > 0 else 0
    exp = expected["sum"]
    print(f"Sum | {exp['polars']:.3f} | {polars_time:.3f} | {exp['arrayops']:.3f} | {arrayops_time:.3f} | {ratio:.1f}x")
    
    # Scale
    series = pl.Series("values", values)
    arr_ao = array.array("i", values)
    arr_copy = array.array("i", values)
    def scale_op():
        arr_copy[:] = arr_ao[:]
        ao.scale(arr_copy, 2.0)
    polars_time = benchmark_func(lambda: series * 2, iterations=100)
    arrayops_time = benchmark_func(scale_op, iterations=100)
    ratio = polars_time / arrayops_time if arrayops_time > 0 else 0
    exp = expected["scale"]
    print(f"Scale | {exp['polars']:.3f} | {polars_time:.3f} | {exp['arrayops']:.3f} | {arrayops_time:.3f} | {ratio:.1f}x")
    
    # Mean
    series = pl.Series("values", values)
    arr_ao = array.array("i", values)
    polars_time = benchmark_func(lambda: series.mean(), iterations=100)
    arrayops_time = benchmark_func(ao.mean, arr_ao, iterations=100)
    ratio = polars_time / arrayops_time if arrayops_time > 0 else 0
    exp = expected["mean"]
    print(f"Mean | {exp['polars']:.3f} | {polars_time:.3f} | {exp['arrayops']:.3f} | {arrayops_time:.3f} | {ratio:.1f}x")
    
    # Filter
    threshold = size // 2
    series = pl.Series("values", values)
    arr_ao = array.array("i", values)
    polars_time = benchmark_func(lambda: series.filter(series > threshold), iterations=50)
    arrayops_time = benchmark_func(lambda: ao.filter(arr_ao, lambda x: x > threshold), iterations=20)
    ratio = polars_time / arrayops_time if arrayops_time > 0 else 0
    exp = expected["filter"]
    print(f"Filter | {exp['polars']:.3f} | {polars_time:.3f} | {exp['arrayops']:.3f} | {arrayops_time:.3f} | {ratio:.1f}x")
    
    print()


if __name__ == "__main__":
    verify_numpy_benchmarks()
    verify_polars_benchmarks()
    print("Note: Benchmark times may vary by system, but ratios should be similar.")

