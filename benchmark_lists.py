#!/usr/bin/env python3
"""Benchmark arrayops vs Python lists."""

import time
import array
import sys

try:
    import arrayops as ao
except ImportError:
    print("Error: arrayops not installed. Run 'maturin develop' first.")
    sys.exit(1)

def benchmark_sum(size):
    """Benchmark sum operation."""
    # Use smaller values to avoid overflow
    values = [i % 1000 for i in range(size)]
    
    # List
    my_list = list(values)
    start = time.perf_counter()
    result = sum(my_list)
    list_time = (time.perf_counter() - start) * 1000  # Convert to ms
    
    # Array + arrayops
    my_array = array.array('i', values)
    start = time.perf_counter()
    result = ao.sum(my_array)
    array_time = (time.perf_counter() - start) * 1000
    
    return list_time, array_time, result

def benchmark_scale(size):
    """Benchmark scale/multiply operation."""
    values = [i % 1000 for i in range(size)]
    
    # List comprehension
    my_list = list(values)
    start = time.perf_counter()
    result = [x * 2 for x in my_list]
    list_time = (time.perf_counter() - start) * 1000
    
    # Array + arrayops scale
    my_array = array.array('i', values)
    start = time.perf_counter()
    ao.scale(my_array, 2.0)
    array_time = (time.perf_counter() - start) * 1000
    
    return list_time, array_time

def benchmark_map(size):
    """Benchmark map operation."""
    values = [i % 1000 for i in range(size)]
    
    # List comprehension
    my_list = list(values)
    start = time.perf_counter()
    result = [x * 2 for x in my_list]
    list_time = (time.perf_counter() - start) * 1000
    
    # Array + arrayops map
    my_array = array.array('i', values)
    start = time.perf_counter()
    result = ao.map(my_array, lambda x: x * 2)
    array_time = (time.perf_counter() - start) * 1000
    
    return list_time, array_time

def benchmark_filter(size):
    """Benchmark filter operation."""
    values = [i % 1000 for i in range(size)]
    
    # List comprehension
    my_list = list(values)
    start = time.perf_counter()
    result = [x for x in my_list if x % 2 == 0]
    list_time = (time.perf_counter() - start) * 1000
    
    # Array + arrayops filter
    my_array = array.array('i', values)
    start = time.perf_counter()
    result = ao.filter(my_array, lambda x: x % 2 == 0)
    array_time = (time.perf_counter() - start) * 1000
    
    return list_time, array_time

def benchmark_reduce(size):
    """Benchmark reduce operation."""
    from functools import reduce
    
    # List with functools.reduce
    my_list = list(range(1, size + 1))
    start = time.perf_counter()
    result = reduce(lambda acc, x: acc * x, my_list, 1)
    list_time = (time.perf_counter() - start) * 1000
    
    # Array + arrayops reduce
    my_array = array.array('i', range(1, size + 1))
    start = time.perf_counter()
    result = ao.reduce(my_array, lambda acc, x: acc * x, initial=1)
    array_time = (time.perf_counter() - start) * 1000
    
    return list_time, array_time

def get_memory_size(obj):
    """Get memory size of object."""
    return sys.getsizeof(obj)

def run_benchmarks():
    """Run all benchmarks."""
    sizes = [1000, 100_000, 1_000_000]
    
    print("=" * 80)
    print("BENCHMARK: arrayops vs Python Lists")
    print("=" * 80)
    print()
    
    for size in sizes:
        print(f"\nArray Size: {size:,} elements")
        print("-" * 80)
        
        # Sum
        list_time, array_time, _ = benchmark_sum(size)
        speedup = list_time / array_time if array_time > 0 else 0
        print(f"Sum:")
        print(f"  List:     {list_time:8.3f} ms")
        print(f"  arrayops: {array_time:8.3f} ms")
        print(f"  Speedup:  {speedup:6.1f}x")
        
        # Scale
        list_time, array_time = benchmark_scale(size)
        speedup = list_time / array_time if array_time > 0 else 0
        print(f"Scale (multiply by 2):")
        print(f"  List:     {list_time:8.3f} ms")
        print(f"  arrayops: {array_time:8.3f} ms")
        print(f"  Speedup:  {speedup:6.1f}x")
        
        # Map
        list_time, array_time = benchmark_map(size)
        speedup = list_time / array_time if array_time > 0 else 0
        print(f"Map (x * 2):")
        print(f"  List:     {list_time:8.3f} ms")
        print(f"  arrayops: {array_time:8.3f} ms")
        print(f"  Speedup:  {speedup:6.1f}x")
        
        # Filter
        list_time, array_time = benchmark_filter(size)
        speedup = list_time / array_time if array_time > 0 else 0
        print(f"Filter (even numbers):")
        print(f"  List:     {list_time:8.3f} ms")
        print(f"  arrayops: {array_time:8.3f} ms")
        print(f"  Speedup:  {speedup:6.1f}x")
        
        # Reduce
        list_time, array_time = benchmark_reduce(min(size, 10000))  # Limit reduce size
        speedup = list_time / array_time if array_time > 0 else 0
        print(f"Reduce (product):")
        print(f"  List:     {list_time:8.3f} ms")
        print(f"  arrayops: {array_time:8.3f} ms")
        print(f"  Speedup:  {speedup:6.1f}x")
    
    # Memory comparison
    print("\n" + "=" * 80)
    print("MEMORY COMPARISON")
    print("=" * 80)
    print()
    
    for size in [1_000_000]:
        my_list = list(range(size))
        my_array = array.array('i', range(size))
        
        list_size = get_memory_size(my_list)
        array_size = get_memory_size(my_array)
        savings = (1 - array_size / list_size) * 100
        
        print(f"Array Size: {size:,} elements (int32)")
        print(f"  List:     {list_size / (1024*1024):8.2f} MB")
        print(f"  array:    {array_size / (1024*1024):8.2f} MB")
        print(f"  Savings:  {savings:6.1f}%")

if __name__ == "__main__":
    run_benchmarks()

