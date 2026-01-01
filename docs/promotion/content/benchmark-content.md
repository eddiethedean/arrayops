# Performance Benchmark Content

This document provides benchmark methodology, results presentation formats, and content for performance comparisons involving arrayops.

## Benchmark Methodology

### Testing Environment

**Standard Environment Specification**:
```markdown
## Testing Environment

- **Python Version**: 3.11.0
- **OS**: macOS 13.0 (or Linux/Windows)
- **CPU**: [CPU model] ([cores] cores)
- **RAM**: [RAM amount]
- **Python Implementation**: CPython
- **arrayops Version**: 1.0.0
- **NumPy Version**: 1.24.0 (for comparisons)
```

### Benchmark Setup

**Standard Benchmark Setup**:
```python
import time
import array
import arrayops as ao
import numpy as np

def benchmark_sum_pure_python(arr, iterations=100):
    """Benchmark pure Python sum()"""
    times = []
    for _ in range(iterations):
        start = time.perf_counter()
        result = sum(arr)
        end = time.perf_counter()
        times.append(end - start)
    return min(times) * 1000  # Convert to milliseconds

def benchmark_sum_arrayops(arr, iterations=100):
    """Benchmark arrayops.sum()"""
    times = []
    for _ in range(iterations):
        start = time.perf_counter()
        result = ao.sum(arr)
        end = time.perf_counter()
        times.append(end - start)
    return min(times) * 1000

def benchmark_sum_numpy(arr, iterations=100):
    """Benchmark NumPy sum()"""
    np_arr = np.array(arr)
    times = []
    for _ in range(iterations):
        start = time.perf_counter()
        result = np.sum(np_arr)
        end = time.perf_counter()
        times.append(end - start)
    return min(times) * 1000

# Test data
arr = array.array('i', range(1_000_000))

# Run benchmarks
python_time = benchmark_sum_pure_python(arr)
arrayops_time = benchmark_sum_arrayops(arr)
numpy_time = benchmark_sum_numpy(arr)

print(f"Pure Python: {python_time:.2f}ms")
print(f"arrayops: {arrayops_time:.2f}ms")
print(f"NumPy: {numpy_time:.2f}ms")
print(f"Speedup (arrayops): {python_time/arrayops_time:.1f}x")
```

### Measurement Best Practices

1. **Use `time.perf_counter()`**: Most accurate for benchmarks
2. **Multiple Iterations**: Run multiple times, use minimum or median
3. **Warm-up Runs**: Discard first few runs (cold start)
4. **Consistent Conditions**: Same machine, no other heavy processes
5. **Report Methodology**: Always include methodology details

## Results Presentation Format

### Table Format

**Standard Results Table**:
```markdown
| Operation | Array Size | Pure Python | arrayops | NumPy | Speedup |
|-----------|------------|-------------|----------|-------|---------|
| sum()     | 1M int32   | 50.2 ms     | 0.5 ms   | 1.1 ms| 100x    |
| sum()     | 10M int32  | 520.0 ms    | 5.2 ms   | 11.0 ms| 100x    |
| scale()   | 1M int32   | 80.5 ms     | 1.5 ms   | 2.3 ms | 54x     |
| scale()   | 10M int32  | 810.0 ms    | 15.0 ms  | 23.0 ms| 54x     |
```

### Chart Description Format

**Bar Chart Description**:
```markdown
## Performance Comparison: Sum Operation

The following chart shows execution time for summing 1 million int32 
elements:

- Pure Python: 50.2ms
- arrayops: 0.5ms (100x faster)
- NumPy: 1.1ms (45x faster than Python)

[Insert bar chart here]

Key observations:
- arrayops provides the fastest execution time
- Significant speedup over pure Python
- Comparable to NumPy for 1D operations
```

### Text Results Format

**Detailed Results Description**:
```markdown
## Benchmark Results

### Sum Operation (1M int32 elements)

**Pure Python (`sum()`)**:
- Execution time: 50.2ms (average of 100 runs, minimum reported)
- Memory: Standard Python overhead
- Notes: Standard library implementation

**arrayops (`ao.sum()`)**:
- Execution time: 0.5ms (average of 100 runs, minimum reported)
- Speedup: 100x faster than pure Python
- Memory: Zero-copy, minimal overhead
- Notes: Rust-accelerated implementation

**NumPy (`np.sum()`)**:
- Execution time: 1.1ms (average of 100 runs, minimum reported)
- Speedup: 45x faster than pure Python
- Memory: NumPy array overhead
- Notes: Includes conversion from array.array to numpy.ndarray

**Analysis**:
arrayops provides the best performance for this operation, achieving 
100x speedup over pure Python and approximately 2x faster than NumPy 
(including conversion overhead). The zero-copy implementation ensures 
minimal memory overhead.
```

## Comparison Charts Structure

### Chart Types

1. **Bar Charts**: Compare execution times
2. **Line Charts**: Show scaling with array size
3. **Speedup Charts**: Show speedup factors
4. **Memory Charts**: Compare memory usage

### Chart Creation Guidelines

**Bar Chart Example Structure**:
```python
import matplotlib.pyplot as plt
import numpy as np

# Data
operations = ['sum()', 'scale()', 'mean()']
python_times = [50.2, 80.5, 55.0]
arrayops_times = [0.5, 1.5, 0.6]
numpy_times = [1.1, 2.3, 1.2]

# Create chart
x = np.arange(len(operations))
width = 0.25

fig, ax = plt.subplots(figsize=(10, 6))
bars1 = ax.bar(x - width, python_times, width, label='Pure Python')
bars2 = ax.bar(x, arrayops_times, width, label='arrayops')
bars3 = ax.bar(x + width, numpy_times, width, label='NumPy')

ax.set_ylabel('Execution Time (ms)')
ax.set_title('Performance Comparison: 1M int32 Array Operations')
ax.set_xticks(x)
ax.set_xticklabels(operations)
ax.legend()
ax.set_yscale('log')  # Log scale for better visualization

plt.tight_layout()
plt.savefig('performance_comparison.png', dpi=300)
```

### Chart Presentation Notes

- Use log scale when values vary widely
- Include clear labels and legend
- Use consistent colors across charts
- Include methodology note on charts
- Export high-resolution (300 DPI minimum)

## Code Examples for Benchmarks

### Complete Benchmark Script

```python
#!/usr/bin/env python3
"""
Comprehensive benchmark script for arrayops operations.
"""

import time
import array
import arrayops as ao
import numpy as np
from typing import List, Tuple

def benchmark_operation(
    name: str,
    python_func,
    arrayops_func=None,
    numpy_func=None,
    array_size: int = 1_000_000,
    iterations: int = 100
) -> Tuple[float, float, float]:
    """Benchmark an operation across implementations."""
    
    # Prepare test data
    test_arr = array.array('i', range(array_size))
    
    # Warm-up
    if python_func:
        python_func(test_arr)
    if arrayops_func:
        arrayops_func(test_arr)
    if numpy_func:
        numpy_arr = np.array(test_arr)
        numpy_func(numpy_arr)
    
    # Benchmark Python
    python_time = None
    if python_func:
        times = []
        for _ in range(iterations):
            start = time.perf_counter()
            python_func(test_arr)
            times.append(time.perf_counter() - start)
        python_time = min(times) * 1000  # ms
    
    # Benchmark arrayops
    arrayops_time = None
    if arrayops_func:
        times = []
        for _ in range(iterations):
            start = time.perf_counter()
            arrayops_func(test_arr)
            times.append(time.perf_counter() - start)
        arrayops_time = min(times) * 1000  # ms
    
    # Benchmark NumPy
    numpy_time = None
    if numpy_func:
        numpy_arr = np.array(test_arr)
        times = []
        for _ in range(iterations):
            start = time.perf_counter()
            numpy_func(numpy_arr)
            times.append(time.perf_counter() - start)
        numpy_time = min(times) * 1000  # ms
    
    return python_time, arrayops_time, numpy_time

def main():
    """Run comprehensive benchmarks."""
    
    print("Running arrayops benchmarks...")
    print("=" * 60)
    
    # Benchmark sum
    python_time, arrayops_time, numpy_time = benchmark_operation(
        "sum",
        python_func=lambda arr: sum(arr),
        arrayops_func=lambda arr: ao.sum(arr),
        numpy_func=lambda arr: np.sum(arr)
    )
    
    print(f"\nSum Operation (1M int32):")
    print(f"  Pure Python: {python_time:.2f}ms")
    print(f"  arrayops:    {arrayops_time:.2f}ms ({python_time/arrayops_time:.1f}x faster)")
    print(f"  NumPy:       {numpy_time:.2f}ms ({python_time/numpy_time:.1f}x faster)")
    
    # Benchmark scale
    python_time, arrayops_time, numpy_time = benchmark_operation(
        "scale",
        python_func=lambda arr: [x * 2.0 for x in arr],
        arrayops_func=lambda arr: ao.scale(arr, 2.0),
        numpy_func=lambda arr: np.multiply(arr, 2.0, out=arr)
    )
    
    print(f"\nScale Operation (1M int32):")
    print(f"  Pure Python: {python_time:.2f}ms")
    print(f"  arrayops:    {arrayops_time:.2f}ms ({python_time/arrayops_time:.1f}x faster)")
    print(f"  NumPy:       {numpy_time:.2f}ms ({python_time/numpy_time:.1f}x faster)")

if __name__ == "__main__":
    main()
```

## Interpretation Guidelines

### Performance Analysis Framework

1. **Absolute Performance**: Raw execution times
2. **Relative Performance**: Speedup factors
3. **Scaling**: How performance changes with array size
4. **Memory**: Memory usage and efficiency
5. **Trade-offs**: Performance vs. other factors

### Common Interpretations

**High Speedup (50x+)**:
- Significant performance improvement
- Worth considering for performance-critical code
- May be worth the dependency (even if zero)

**Moderate Speedup (10-50x)**:
- Good performance improvement
- Consider use case and requirements
- Evaluate trade-offs

**Low Speedup (<10x)**:
- Modest improvement
- Consider overhead and use case
- May not be worth it for small arrays

### Context Considerations

- **Array Size**: Performance characteristics vary with size
- **Use Case**: Performance needs vary by use case
- **Dependencies**: Zero dependencies is valuable
- **Integration**: Ease of integration matters
- **Maintenance**: Long-term maintenance considerations

## Visual Representation Suggestions

### Chart Recommendations

1. **Bar Charts**: For comparing specific operations
2. **Line Charts**: For showing scaling behavior
3. **Log Scale**: When values vary widely
4. **Grouped Bars**: For comparing multiple implementations
5. **Heatmaps**: For multi-dimensional comparisons

### Presentation Tips

- Use consistent color scheme
- Include clear labels and units
- Add methodology notes
- Export high-resolution
- Consider accessibility (color-blind friendly)

## Example Benchmark Content

See platform-specific guides for examples:
- Reddit: Performance benchmark posts
- Hacker News: Technical benchmark articles
- Dev.to: Benchmark tutorial articles
- Blog posts: Comprehensive benchmark analyses

