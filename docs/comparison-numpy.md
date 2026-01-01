# arrayops vs NumPy: When to Use Each

A concise comparison for choosing between `arrayops` with `array.array` and NumPy for 1D numeric data.

**Note:** Benchmarks from macOS ARM64, Python 3.12.

## Quick Decision

**Use NumPy when:**
- Multi-dimensional arrays (2D, 3D, etc.)
- Broadcasting, advanced indexing
- Scientific computing, ML, data science
- Ecosystem integration (scipy, pandas, matplotlib, sklearn)
- Specialized functions (FFT, linear algebra)

**Use `array.array` + `arrayops` when:**
- 1D numeric data only
- Memory efficiency critical
- Zero/minimal dependencies required
- Binary I/O and C interop needed
- Lightweight scripts and system tools

## Key Differences

### Memory Efficiency (1M int32 elements)

| Type | Memory Usage | Overhead |
|------|--------------|----------|
| `array.array` | 3.90 MB | Minimal |
| NumPy `ndarray` | 7.63 MB | ~3.73 MB |

Arrays use ~49% less memory than NumPy for equivalent data.

### Dependencies

- **NumPy**: ~20-30 MB package with BLAS/LAPACK dependencies
- **arrayops**: ~1-2 MB, zero dependencies (Rust stdlib only)

### Performance Benchmarks

*Benchmark results vary significantly by system, Python version, and library versions. The following are representative values from macOS ARM64, Python 3.12. Use `scripts/benchmark_numpy.py` to run benchmarks on your system.*

| Operation | Size | NumPy | arrayops | Ratio |
|-----------|------|-------|----------|-------|
| Sum | 1M | 0.297 ms | 7.519 ms | 25x faster |
| Scale (multiply) | 1M | 0.300 ms | 4.763 ms | 16x faster |
| Mean | 1M | 0.307 ms | 7.132 ms | 23x faster |
| Filter (boolean) | 1M | 3.321 ms | 204.230 ms | 62x faster |

**NumPy is significantly faster** for vectorized operations and boolean indexing. **arrayops advantages**: native Python callable support (map/filter with lambdas), simpler API, zero dependencies, better memory efficiency.

### Features

**NumPy**: Multi-dimensional arrays, broadcasting, advanced indexing, linear algebra, FFT, extensive ecosystem.

**arrayops**: 1D operations only:
- Basic: `sum()`, `mean()`, `min()`, `max()`, `scale()`
- Stats: `std()`, `var()`, `median()`
- Transform: `map()`, `filter()`, `reduce()`
- Element-wise: `add()`, `multiply()`, `clip()`, `normalize()`
- Manipulation: `sort()`, `reverse()`, `unique()`

No multi-dimensional support, broadcasting, or advanced indexing.

### API Style

**NumPy**: Vectorized operations with broadcasting
```python
arr * 2  # Vectorized multiplication
arr[arr > 500]  # Boolean indexing
np.sum(arr)  # Function-based
```

**arrayops**: Python function-based
```python
ao.scale(arr, 2.0)  # In-place scaling
ao.filter(arr, lambda x: x > 500)  # Python callable
ao.sum(arr)  # Function-based
```

## Decision Matrix

| Requirement | NumPy | arrayops |
|-------------|-------|----------|
| Multi-dimensional arrays | ✅ Essential | ❌ 1D only |
| Memory efficiency | ✅ Good | ✅ Excellent |
| Zero dependencies | ❌ Large | ✅ Minimal |
| Binary I/O simplicity | ⚠️ Good | ✅ Excellent |
| Vectorized operations | ✅ Excellent | ⚠️ Limited |
| Python callables (map/filter) | ⚠️ Needs vectorize | ✅ Native |
| Scientific computing ecosystem | ✅ Central | ❌ Convert to NumPy |
| Learning curve | ⚠️ Steep | ✅ Gentle |
| C interop | ✅ Good | ✅ Excellent |
| Embedded systems | ❌ Too large | ✅ Suitable |

## Example: Same Task

**Task:** Process sensor data, calculate mean/std, filter outliers

```python
# NumPy
import numpy as np
data = np.fromfile('sensor.bin', dtype=np.float32)
mean = np.mean(data)
std = np.std(data)
filtered = data[(data >= mean - 3*std) & (data <= mean + 3*std)]  # Boolean indexing

# arrayops
import array
import arrayops as ao
data = array.array('f')
with open('sensor.bin', 'rb') as f:
    data.fromfile(f, 1_000_000)  # Direct binary I/O
mean = ao.mean(data)
std = ao.std(data)
filtered = ao.filter(data, lambda x: mean - 3*std <= x <= mean + 3*std)  # Python callable
```

## Migration

```python
# NumPy → arrayops (1D only)
np.sum(arr) → ao.sum(array.array('i', arr.tolist()))
arr[arr > 500] → ao.filter(array.array('i', arr.tolist()), lambda x: x > 500)

# arrayops → NumPy
ao.sum(arr) → np.sum(np.array(arr))
```

## Conclusion

**NumPy** excels at multi-dimensional arrays, broadcasting, and scientific computing. **arrayops** excels at lightweight 1D array processing with zero dependencies, native Python callable support, and better memory efficiency.

Choose NumPy for versatility and performance. Choose arrayops for 1D numeric data when memory efficiency, minimal dependencies, or simplicity matter more than raw performance.
