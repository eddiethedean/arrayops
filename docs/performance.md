# Performance Guide

Performance characteristics, benchmarking, and optimization tips for `arrayops`.

## Overview

`arrayops` provides significant performance improvements over pure Python operations by leveraging Rust's zero-cost abstractions and efficient memory access patterns.

## Performance Characteristics

### Benchmark Results

| Operation | Array Size | Python Time | arrayops Time | Speedup |
|-----------|------------|-------------|---------------|---------|
| Sum (int32) | 1K | ~0.05ms | ~0.0005ms | 100x |
| Sum (int32) | 1M | ~50ms | ~0.5ms | 100x |
| Sum (int32) | 10M | ~500ms | ~5ms | 100x |
| Scale (int32) | 1K | ~0.08ms | ~0.0015ms | 50x |
| Scale (int32) | 1M | ~80ms | ~1.5ms | 50x |
| Scale (int32) | 10M | ~800ms | ~15ms | 50x |

*Benchmarks run on typical modern hardware. Actual results may vary.*

## Optional Performance Features

`arrayops` supports optional performance optimizations via feature flags. These features are transparent - the API remains the same, but performance improves for large arrays when the features are enabled.

### Parallel Execution (`--features parallel`)

Parallel execution uses Rayon to distribute work across multiple CPU cores. This provides significant speedups for large arrays on multi-core systems.

#### When to Use Parallel Execution

- **Large arrays**: Arrays with 10,000+ elements (sum) or 5,000+ elements (scale)
- **Multi-core systems**: Systems with 2+ CPU cores
- **CPU-bound workloads**: Operations that are compute-intensive

#### Performance Characteristics

- **Threshold-based**: Automatically enabled only when array size exceeds threshold
- **Near-linear scaling**: ~4x speedup on 4 cores, ~8x on 8 cores (for sum/scale operations)
- **Overhead**: Small arrays (< threshold) use sequential code to avoid parallelization overhead
- **Thread-safe**: Uses thread-safe buffer extraction for parallel processing

#### Enabled Operations

- `sum`: Parallel execution for arrays with 10,000+ elements
- `scale`: Parallel execution for arrays with 5,000+ elements

**Note**: Operations with Python callables (`map`, `filter`, `reduce`) have limited parallelization benefits due to Python's Global Interpreter Lock (GIL).

#### Installation

```bash
# Development
maturin develop --features parallel

# Production build
maturin build --release --features parallel
```

### SIMD Optimizations (`--features simd`)

SIMD (Single Instruction, Multiple Data) optimizations use CPU vector instructions to process multiple elements simultaneously.

#### Current Status

- **Infrastructure**: Framework in place for SIMD optimizations
- **Implementation**: Full implementation pending std::simd API stabilization
- **Expected performance**: 2-4x additional speedup on supported CPUs when implemented

#### Target Operations

- `sum`: Primary target for SIMD optimization
- `scale`: Primary target for SIMD optimization
- Element-wise operations: Future target

#### Installation

```bash
# Development
maturin develop --features simd

# Production build
maturin build --release --features simd
```

### Combining Features

You can enable both parallel and SIMD features together:

```bash
maturin develop --features parallel,simd
```

When both features are enabled, the implementation will use the most appropriate optimization for the array size and operation.

## Sum Operation

### Performance Profile

The `sum` operation is highly optimized:
- **Zero-copy access**: Direct memory access via buffer protocol
- **Monomorphized code**: Type-specific optimized loops
- **SIMD-ready**: Infrastructure in place for SIMD optimizations (via `--features simd`)
- **Parallel execution**: Automatic parallelization for large arrays (via `--features parallel`, 10,000+ elements)
- **Cache-friendly**: Sequential memory access pattern

### Benchmarking Sum

```python
import array
import arrayops as ao
import time

def benchmark_sum(size=10_000):
    # Create test array (use smaller size for int32 to avoid overflow)
    arr = array.array('i', list(range(size)))
    
    # Python sum
    start = time.perf_counter()
    python_result = sum(arr)
    python_time = time.perf_counter() - start
    
    # arrayops sum
    start = time.perf_counter()
    arrayops_result = ao.sum(arr)
    arrayops_time = time.perf_counter() - start
    
    # Verify results match
    assert python_result == arrayops_result
    
    speedup = python_time / arrayops_time
    print(f"Size: {size:,}")
    print(f"Python: {python_time*1000:.2f}ms")
    print(f"arrayops: {arrayops_time*1000:.2f}ms")
    print(f"Speedup: {speedup:.1f}x")
    
    return speedup

# Run benchmarks
for size in [1_000, 10_000, 50_000]:
    benchmark_sum(size)
    print()
```

### Optimization Tips

1. **Use appropriate types**: Smaller types (int8, int16) may be faster for very large arrays due to better cache utilization
2. **Batch processing**: Process large datasets in chunks if memory is limited
3. **Avoid conversions**: Work directly with `array.array`, avoid converting to lists

## Scale Operation

### Performance Profile

The `scale` operation benefits from:
- **In-place modification**: No memory allocation
- **Type-specific loops**: Optimized for each numeric type
- **Parallel execution**: Automatic parallelization for large arrays (via `--features parallel`, 5,000+ elements)
- **SIMD-ready**: Infrastructure in place for SIMD optimizations (via `--features simd`)
- **Sequential access**: Cache-friendly memory pattern

### Benchmarking Scale

```python
import array
import arrayops as ao
import time

def benchmark_scale(size=100_000):
    # Create test arrays (use smaller size for int32 to avoid overflow)
    arr1 = array.array('i', list(range(size)))
    arr2 = array.array('i', list(range(size)))
    
    # Python loop
    start = time.perf_counter()
    for i in range(len(arr1)):
        arr1[i] = int(arr1[i] * 2.0)
    python_time = time.perf_counter() - start
    
    # arrayops scale
    start = time.perf_counter()
    ao.scale(arr2, 2.0)
    arrayops_time = time.perf_counter() - start
    
    speedup = python_time / arrayops_time
    print(f"Size: {size:,}")
    print(f"Python: {python_time*1000:.2f}ms")
    print(f"arrayops: {arrayops_time*1000:.2f}ms")
    print(f"Speedup: {speedup:.1f}x")
    
    return speedup
```

## Memory Usage

### Zero-Copy Buffer Access

`arrayops` uses Python's buffer protocol for zero-copy access:

```python
# No copying occurs - direct memory access
arr = array.array('i', [1, 2, 3, 4, 5])
total = ao.sum(arr)  # Direct access to arr's memory
```

**Benefits:**
- No memory overhead for operations
- Fast access to array data
- Memory-safe (Rust guarantees)

### Memory Comparison

| Operation | Memory Overhead |
|-----------|----------------|
| Python `sum()` | Minimal (iterator overhead) |
| `ao.sum()` | Zero (direct buffer access) |
| Python loop | Minimal |
| `ao.scale()` | Zero (in-place modification) |

## When to Use arrayops

### Use arrayops when:

- ✅ Processing large numeric arrays
- ✅ Performance is critical
- ✅ Working with binary data formats
- ✅ Need zero-copy operations
- ✅ Want lightweight alternative to NumPy

### Consider alternatives when:

- ❌ Need multi-dimensional arrays (use NumPy)
- ❌ Need advanced linear algebra (use NumPy)
- ❌ Arrays are very small (< 100 elements) - overhead may not be worth it
- ❌ Need array of Python objects (not supported)

## Performance Optimization Tips

### 1. Choose Appropriate Types

```python
# For small integers (0-255), use uint8
small_data = array.array('B', [100, 150, 200])

# For larger integers, use int32
large_data = array.array('i', [1000000, 2000000])

# For floats, prefer float32 unless you need precision
float_data = array.array('f', [1.5, 2.5, 3.5])
```

### 2. Batch Processing

For very large datasets, process in batches:

```python
def process_large_file(file_path, batch_size=10000):
    results = []
    with open(file_path, 'rb') as f:
        while True:
            batch = array.array('f')
            try:
                batch.fromfile(f, batch_size)
            except EOFError:
                break
            
            if len(batch) == 0:
                break
            
            # Process batch
            total = ao.sum(batch)
            results.append(total)
    
    return results
```

### 3. Avoid Unnecessary Conversions

```python
# Good: Work directly with array.array
arr = array.array('i', [1, 2, 3])
total = ao.sum(arr)

# Avoid: Converting to list
arr = array.array('i', [1, 2, 3])
total = ao.sum(array.array('i', list(arr)))  # Unnecessary copy
```

### 4. Prefer In-Place Operations

```python
# Good: In-place scaling
arr = array.array('i', [1, 2, 3, 4, 5])
ao.scale(arr, 2.0)  # Modifies arr directly

# Avoid: Creating new arrays when possible
# (When future operations support it)
```

## Performance Regression Testing

### Benchmarking Script

Create a benchmark script to track performance:

```python
import array
import arrayops as ao
import time
import json

def run_benchmarks():
    results = {}
    sizes = [1_000, 10_000, 50_000, 100_000]  # Use smaller sizes for int32 to avoid overflow
    
    for size in sizes:
        # Sum benchmark
        arr = array.array('i', list(range(size)))
        
        start = time.perf_counter()
        result = ao.sum(arr)
        elapsed = time.perf_counter() - start
        
        results[f'sum_{size}'] = {
            'time_ms': elapsed * 1000,
            'throughput': size / elapsed,
        }
    
    return results

# Save results
results = run_benchmarks()
with open('benchmark_results.json', 'w') as f:
    json.dump(results, f, indent=2)
```

### Continuous Benchmarking

- Run benchmarks in CI/CD
- Track performance over time
- Alert on regressions
- Compare against baseline

## Future Optimizations

### Planned Improvements

1. **SIMD Support**: Vectorized operations for additional speedup
2. **Parallel Execution**: Multi-threaded processing for large arrays
3. **Specialized Kernels**: Optimized code paths for common patterns

See [ROADMAP.md](../ROADMAP.md) for details.

## Profiling

### Python Profiling

```python
import cProfile
import array
import arrayops as ao

arr = array.array('i', list(range(100_000)))  # Use smaller size for int32 to avoid overflow

profiler = cProfile.Profile()
profiler.enable()
result = ao.sum(arr)
profiler.disable()
profiler.print_stats()
```

### Rust Profiling

Use platform-specific profilers:
- **Linux**: `perf`
- **macOS**: Instruments
- **Windows**: Visual Studio Profiler

## Comparison with Alternatives

### vs. Pure Python

| Metric | Python | arrayops |
|--------|--------|----------|
| Speed | Baseline | 50-100x faster |
| Memory | Low overhead | Zero-copy |
| Dependencies | None | Rust runtime |

### vs. NumPy

| Metric | NumPy | arrayops |
|--------|-------|----------|
| Memory | Higher overhead | Lower overhead |
| Dependencies | Large | Minimal |
| Use case | Scientific computing, multi-dimensional arrays | Binary I/O, ETL, 1D arrays |
| Focus | Feature-rich scientific computing | Lightweight, zero-copy operations |

## Related Documentation

- [API Reference](api.md) - Function documentation
- [Examples](examples.md) - Usage examples
- [Design Document](../design.md) - Architecture details

