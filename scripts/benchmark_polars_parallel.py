#!/usr/bin/env python3
"""Benchmark arrayops vs Polars for 1D operations, including parallel benchmarks."""

import time
import array
import sys

try:
    import arrayops as ao
except ImportError:
    print("Error: arrayops not installed. Run 'maturin develop' first.")
    sys.exit(1)

try:
    import polars as pl
except ImportError:
    print("Error: Polars not installed. Install with 'pip install polars'")
    sys.exit(1)


def benchmark_func(func, *args, iterations=100, warmup=10):
    """Benchmark a function with warmup and multiple iterations, return min time in ms."""
    # Warmup
    for _ in range(warmup):
        func(*args)
    
    # Benchmark - collect all times
    times = []
    for _ in range(iterations):
        start = time.perf_counter()
        result = func(*args)
        end = time.perf_counter()
        times.append((end - start) * 1000)  # Convert to ms
    
    return min(times)  # Use minimum (best case)


def benchmark_sum(size):
    """Benchmark sum operation."""
    values = [i % 1000 for i in range(size)]
    
    # Polars Series
    series = pl.Series("values", values)
    polars_time = benchmark_func(lambda: series.sum(), iterations=100)
    
    # Array + arrayops
    arr_ao = array.array("i", values)
    arrayops_time = benchmark_func(lambda: ao.sum(arr_ao), iterations=100)
    
    return polars_time, arrayops_time


def benchmark_scale(size):
    """Benchmark scale/multiply operation."""
    values = [i % 1000 for i in range(size)]
    
    # Polars Series vectorized multiply
    series = pl.Series("values", values)
    polars_time = benchmark_func(lambda: (series * 2), iterations=100)
    
    # Array + arrayops scale (in-place)
    arr_ao = array.array("i", values)
    arr_copy = array.array("i", values)  # Copy for each iteration
    def scale_op():
        arr_copy[:] = arr_ao[:]  # Reset
        ao.scale(arr_copy, 2.0)
    arrayops_time = benchmark_func(scale_op, iterations=100)
    
    return polars_time, arrayops_time


def benchmark_mean(size):
    """Benchmark mean operation."""
    values = [i % 1000 for i in range(size)]
    
    # Polars Series
    series = pl.Series("values", values)
    polars_time = benchmark_func(lambda: series.mean(), iterations=100)
    
    # Array + arrayops
    arr_ao = array.array("i", values)
    arrayops_time = benchmark_func(lambda: ao.mean(arr_ao), iterations=100)
    
    return polars_time, arrayops_time


def benchmark_filter(size):
    """Benchmark filter operation."""
    values = [i % 1000 for i in range(size)]
    threshold = size // 2
    
    # Polars Series filter (expression-based)
    series = pl.Series("values", values)
    polars_time = benchmark_func(lambda: series.filter(series > threshold), iterations=50)
    
    # Array + arrayops filter (Python callable)
    arr_ao = array.array("i", values)
    arrayops_time = benchmark_func(lambda: ao.filter(arr_ao, lambda x: x > threshold), iterations=20)
    
    return polars_time, arrayops_time


def benchmark_std(size):
    """Benchmark standard deviation operation."""
    values = [i % 1000 for i in range(size)]
    
    # Polars Series
    series = pl.Series("values", values)
    polars_time = benchmark_func(lambda: series.std(), iterations=100)
    
    # Array + arrayops
    arr_ao = array.array("i", values)
    arrayops_time = benchmark_func(lambda: ao.std(arr_ao), iterations=100)
    
    return polars_time, arrayops_time


def get_memory_size_polars(obj):
    """Get memory size of Polars object."""
    if isinstance(obj, pl.Series):
        return obj.estimated_size(unit="mb") * 1024 * 1024  # Convert MB to bytes
    elif isinstance(obj, pl.DataFrame):
        return obj.estimated_size(unit="mb") * 1024 * 1024
    return sys.getsizeof(obj)


def run_benchmarks():
    """Run all benchmarks."""
    sizes = [1_000_000]  # Focus on 1M for the comparison doc
    
    print("=" * 80)
    print("BENCHMARK: arrayops vs Polars (1D Series) - With Parallel Features")
    print("=" * 80)
    print()
    
    for size in sizes:
        print(f"\nArray Size: {size:,} elements")
        print("-" * 80)
        
        # Sum
        polars_time, arrayops_time = benchmark_sum(size)
        ratio = polars_time / arrayops_time if arrayops_time > 0 else 0
        print("Sum:")
        print(f"  Polars:   {polars_time:8.3f} ms")
        print(f"  arrayops: {arrayops_time:8.3f} ms")
        print(f"  Ratio:    {ratio:6.2f}x (Polars/arrayops)")
        
        # Scale
        polars_time, arrayops_time = benchmark_scale(size)
        ratio = polars_time / arrayops_time if arrayops_time > 0 else 0
        print("Scale (multiply by 2):")
        print(f"  Polars:   {polars_time:8.3f} ms")
        print(f"  arrayops: {arrayops_time:8.3f} ms")
        print(f"  Ratio:    {ratio:6.2f}x (Polars/arrayops)")
        
        # Mean
        polars_time, arrayops_time = benchmark_mean(size)
        ratio = polars_time / arrayops_time if arrayops_time > 0 else 0
        print("Mean:")
        print(f"  Polars:   {polars_time:8.3f} ms")
        print(f"  arrayops: {arrayops_time:8.3f} ms")
        print(f"  Ratio:    {ratio:6.2f}x (Polars/arrayops)")
        
        # Filter
        polars_time, arrayops_time = benchmark_filter(size)
        ratio = polars_time / arrayops_time if arrayops_time > 0 else 0
        print("Filter (values > threshold):")
        print(f"  Polars:   {polars_time:8.3f} ms")
        print(f"  arrayops: {arrayops_time:8.3f} ms")
        print(f"  Ratio:    {ratio:6.2f}x (Polars/arrayops)")


if __name__ == "__main__":
    run_benchmarks()

