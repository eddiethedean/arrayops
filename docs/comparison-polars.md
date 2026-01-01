# arrayops vs Polars: When to Use Each

A concise comparison for choosing between `arrayops` with `array.array` and Polars for 1D numeric data.

**Note:** Benchmarks from macOS ARM64, Python 3.12. Polars comparisons use Series (1D) for fair comparison.

## Quick Decision

**Use Polars when:**
- Multi-column DataFrames, joins, groupby
- SQL-like queries with optimization
- Tabular data (CSV, Parquet, databases)
- Time-series operations, window functions
- Null/missing value handling

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
| Polars Series | ~4.2 MB | ~0.3 MB |
| Polars DataFrame | ~4.5 MB | ~0.6 MB |

Arrays have the lowest overhead for pure 1D numeric data.

### Dependencies

- **Polars**: ~50-100 MB package with Arrow dependencies
- **arrayops**: ~1-2 MB, zero dependencies (Rust stdlib only)

### Performance Benchmarks

*Benchmark results vary significantly by system, Python version, and library versions. The following are representative values from macOS ARM64, Python 3.12. Use `scripts/benchmark_polars.py` to run benchmarks on your system.*

| Operation | Size | Polars Series | arrayops | Ratio |
|-----------|------|---------------|----------|-------|
| Sum | 1M | 0.312 ms | 7.519 ms | 24x faster |
| Scale (multiply) | 1M | 0.425 ms | 4.763 ms | 11x faster |
| Mean | 1M | 0.335 ms | 7.132 ms | 21x faster |
| Filter | 1M | 3.521 ms | 204.230 ms | 58x faster |

**Polars is significantly faster** for vectorized operations, especially expression-based filtering. **arrayops advantages**: native Python callable support (map/filter with lambdas), simpler API, zero dependencies.

### Features

**Polars**: Comprehensive DataFrame operations, query optimization, lazy evaluation, null handling, CSV/Parquet I/O.

**arrayops**: 1D operations only:
- Basic: `sum()`, `mean()`, `min()`, `max()`, `scale()`
- Stats: `std()`, `var()`, `median()`
- Transform: `map()`, `filter()`, `reduce()`
- Element-wise: `add()`, `multiply()`, `clip()`, `normalize()`
- Manipulation: `sort()`, `reverse()`, `unique()`

No DataFrame operations, joins, groupby, or null handling.

### API Style

**Polars**: Expression-based with query optimization
```python
df.lazy().filter(pl.col("values") > 500).with_columns(...).collect()
```

**arrayops**: Python function-based
```python
ao.filter(arr, lambda x: x > 500)
ao.map(arr, lambda x: x * 2)
```

## Decision Matrix

| Requirement | Polars | arrayops |
|-------------|--------|----------|
| Multi-column DataFrames | ✅ | ❌ 1D only |
| Memory efficiency | ✅ Good | ✅ Excellent |
| Zero dependencies | ❌ Large | ✅ Minimal |
| Binary I/O simplicity | ⚠️ Needs conversion | ✅ Excellent |
| Vectorized operations | ✅ Excellent | ⚠️ Limited |
| Python callables (map/filter) | ❌ Expression-based | ✅ Native |
| Query optimization | ✅ Advanced | ❌ None |
| Null handling | ✅ Comprehensive | ❌ No support |
| CSV/Parquet I/O | ✅ Native | ❌ Manual |
| C interop | ⚠️ Good | ✅ Excellent |
| Embedded systems | ❌ Too large | ✅ Suitable |

## Example: Same Task

**Task:** Process 1M sensor values, calculate mean/std, filter outliers

```python
# Polars
import polars as pl
df = pl.DataFrame({"values": values})  # Requires conversion
stats = df.select([pl.col("values").mean(), pl.col("values").std()])
mean_val, std_val = stats[0, 0], stats[0, 1]
filtered = df.filter((pl.col("values") >= mean_val - 3*std_val) & 
                     (pl.col("values") <= mean_val + 3*std_val))

# arrayops
import array
import arrayops as ao
data = array.array('f')
with open('sensor.bin', 'rb') as f:
    data.fromfile(f, 1_000_000)  # Direct binary I/O
mean_val = ao.mean(data)
std_val = ao.std(data)
filtered = ao.filter(data, lambda x: mean_val - 3*std_val <= x <= mean_val + 3*std_val)
```

## Conclusion

**Polars** excels at DataFrame operations, query optimization, and tabular data analysis. **arrayops** excels at lightweight 1D array processing with zero dependencies, native Python callable support, and better memory efficiency.

Choose Polars for data analysis. Choose arrayops for 1D numeric data when memory efficiency, minimal dependencies, or simplicity matter more than raw performance.
