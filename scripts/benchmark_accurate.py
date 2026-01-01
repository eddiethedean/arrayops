#!/usr/bin/env python3
"""Accurate benchmarks using proper methodology (multiple iterations, min time)."""

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
    """Benchmark function with warmup and multiple iterations, return min time in ms."""
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


def run_comparison_benchmarks():
    """Run benchmarks comparing arrayops vs NumPy and Polars."""
    print("=" * 80)
    print("ACCURATE BENCHMARKS: arrayops vs NumPy vs Polars (1M int32 elements)")
    print("=" * 80)
    print("Methodology: Multiple iterations (100), min time reported")
    print("Values: i % 1000 to avoid overflow")
    print()
    
    size = 1_000_000
    values = [i % 1000 for i in range(size)]
    
    results = {}
    
    # Sum
    print("Sum operation:")
    arr_ao = array.array("i", values)
    arrayops_time = benchmark_func(ao.sum, arr_ao, iterations=100)
    results['sum'] = {'arrayops': arrayops_time}
    
    if NUMPY_AVAILABLE:
        arr_np = np.array(values, dtype=np.int32)
        numpy_time = benchmark_func(np.sum, arr_np, iterations=100)
        results['sum']['numpy'] = numpy_time
        ratio = numpy_time / arrayops_time if arrayops_time > 0 else 0
        print(f"  NumPy:    {numpy_time:8.3f} ms (ratio: {ratio:.2f}x NumPy/arrayops)")
        print(f"  arrayops: {arrayops_time:8.3f} ms")
    
    if POLARS_AVAILABLE:
        series = pl.Series("values", values)
        polars_time = benchmark_func(lambda: series.sum(), iterations=100)
        results['sum']['polars'] = polars_time
        ratio = polars_time / arrayops_time if arrayops_time > 0 else 0
        print(f"  Polars:   {polars_time:8.3f} ms (ratio: {ratio:.2f}x Polars/arrayops)")
    
    print()
    
    # Scale/Multiply
    print("Scale (multiply by 2) operation:")
    arr_ao = array.array("i", values)
    arr_copy = array.array("i", values)
    def scale_op():
        arr_copy[:] = arr_ao[:]
        ao.scale(arr_copy, 2.0)
    arrayops_time = benchmark_func(scale_op, iterations=100)
    results['scale'] = {'arrayops': arrayops_time}
    
    if NUMPY_AVAILABLE:
        arr_np = np.array(values, dtype=np.int32)
        numpy_time = benchmark_func(lambda: arr_np * 2, iterations=100)
        results['scale']['numpy'] = numpy_time
        ratio = numpy_time / arrayops_time if arrayops_time > 0 else 0
        print(f"  NumPy:    {numpy_time:8.3f} ms (ratio: {ratio:.2f}x NumPy/arrayops)")
        print(f"  arrayops: {arrayops_time:8.3f} ms")
    
    if POLARS_AVAILABLE:
        series = pl.Series("values", values)
        polars_time = benchmark_func(lambda: series * 2, iterations=100)
        results['scale']['polars'] = polars_time
        ratio = polars_time / arrayops_time if arrayops_time > 0 else 0
        print(f"  Polars:   {polars_time:8.3f} ms (ratio: {ratio:.2f}x Polars/arrayops)")
    
    print()
    
    # Mean
    print("Mean operation:")
    arr_ao = array.array("i", values)
    arrayops_time = benchmark_func(ao.mean, arr_ao, iterations=100)
    results['mean'] = {'arrayops': arrayops_time}
    
    if NUMPY_AVAILABLE:
        arr_np = np.array(values, dtype=np.int32)
        numpy_time = benchmark_func(np.mean, arr_np, iterations=100)
        results['mean']['numpy'] = numpy_time
        ratio = numpy_time / arrayops_time if arrayops_time > 0 else 0
        print(f"  NumPy:    {numpy_time:8.3f} ms (ratio: {ratio:.2f}x NumPy/arrayops)")
        print(f"  arrayops: {arrayops_time:8.3f} ms")
    
    if POLARS_AVAILABLE:
        series = pl.Series("values", values)
        polars_time = benchmark_func(lambda: series.mean(), iterations=100)
        results['mean']['polars'] = polars_time
        ratio = polars_time / arrayops_time if arrayops_time > 0 else 0
        print(f"  Polars:   {polars_time:8.3f} ms (ratio: {ratio:.2f}x Polars/arrayops)")
    
    print()
    
    # Filter
    print("Filter operation (values > threshold):")
    threshold = size // 2
    arr_ao = array.array("i", values)
    arrayops_time = benchmark_func(lambda: ao.filter(arr_ao, lambda x: x > threshold), iterations=20)
    results['filter'] = {'arrayops': arrayops_time}
    
    if NUMPY_AVAILABLE:
        arr_np = np.array(values, dtype=np.int32)
        numpy_time = benchmark_func(lambda: arr_np[arr_np > threshold], iterations=50)
        results['filter']['numpy'] = numpy_time
        ratio = numpy_time / arrayops_time if arrayops_time > 0 else 0
        print(f"  NumPy:    {numpy_time:8.3f} ms (ratio: {ratio:.2f}x NumPy/arrayops)")
        print(f"  arrayops: {arrayops_time:8.3f} ms")
    
    if POLARS_AVAILABLE:
        series = pl.Series("values", values)
        polars_time = benchmark_func(lambda: series.filter(series > threshold), iterations=50)
        results['filter']['polars'] = polars_time
        ratio = polars_time / arrayops_time if arrayops_time > 0 else 0
        print(f"  Polars:   {polars_time:8.3f} ms (ratio: {ratio:.2f}x Polars/arrayops)")
    
    print()
    print("=" * 80)
    print("Summary Table (ms)")
    print("=" * 80)
    print(f"{'Operation':<12} {'arrayops':<12} {'NumPy':<12} {'Polars':<12}")
    print("-" * 80)
    for op in ['sum', 'scale', 'mean', 'filter']:
        ao_t = results[op]['arrayops']
        np_t = results[op].get('numpy', 'N/A')
        pl_t = results[op].get('polars', 'N/A')
        np_str = f"{np_t:.3f}" if isinstance(np_t, float) else "N/A"
        pl_str = f"{pl_t:.3f}" if isinstance(pl_t, float) else "N/A"
        print(f"{op:<12} {ao_t:<12.3f} {np_str:<12} {pl_str:<12}")
    
    return results


if __name__ == "__main__":
    import platform
    print(f"Python: {sys.version.split()[0]}")
    print(f"Platform: {platform.platform()}")
    print()
    run_comparison_benchmarks()

