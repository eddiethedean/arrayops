# arrayops vs Python Lists: When to Use Each

A concise comparison for choosing between `arrayops` with `array.array` and Python's built-in `list` for numeric data.

**Note:** Benchmarks from macOS ARM64, Python 3.12. Use `scripts/benchmark_lists.py` to run benchmarks on your system.

## Quick Decision

**Use Python Lists when:**
- Mixed data types (strings, numbers, objects)
- Small datasets (< 1,000 elements)
- Rich list methods needed (append, extend, insert, remove, etc.)
- Non-numeric data
- Performance isn't critical

**Use `array.array` + `arrayops` when:**
- Large numeric datasets (thousands+ elements)
- Memory efficiency is critical
- Binary I/O and C interop needed
- Processing sensor data, financial data, image pixels
- In-place operations (scale, clip, normalize, reverse, sort)

## Key Differences

### Memory Efficiency (1M int32 elements)

| Type | Memory Usage | Overhead |
|------|--------------|----------|
| `array.array` | 3.90 MB | Minimal |
| Python `list` | 7.63 MB | ~3.73 MB overhead |

Arrays use ~49% less memory than lists for numeric data.

### Performance Benchmarks

*Benchmark results vary significantly by system, Python version, and library versions. The following are representative values from macOS ARM64, Python 3.12. Use `scripts/benchmark_lists.py` to run benchmarks on your system.*

| Operation | Size | List Time | arrayops Time | Winner |
|-----------|------|-----------|---------------|--------|
| Sum | 1M | 2.762 ms | 0.082 ms | **arrayops** (33.7x faster) |
| Scale (multiply) | 1M | 15.051 ms | 0.089 ms | **arrayops** (169x faster) |
| Map (x * 2) | 1M | 15.247 ms | 62.254 ms | **Lists** (4.1x faster) |
| Filter (even) | 1M | 16.162 ms | 54.508 ms | **Lists** (3.4x faster) |

**Key Insights:**
- **arrayops is significantly faster** for sum and scale operations
- **Lists are faster** for map/filter with Python callables (due to Python callable overhead in arrayops)
- **Memory savings**: Arrays use ~49% less memory

### Data Type Flexibility

**Lists**: Mixed types supported
```python
my_list = [1, "hello", 3.14, [1, 2, 3]]
```

**Arrays**: Single numeric type only
```python
int_array = array.array('i', [1, 2, 3, 4, 5])  # int32 only
```

### Binary I/O

**Lists**: Manual conversion required
```python
import struct
data = []
with open('data.bin', 'rb') as f:
    while True:
        chunk = f.read(4)
        if len(chunk) < 4: break
        data.append(struct.unpack('f', chunk)[0])
```

**Arrays**: Native binary support
```python
data = array.array('f')
with open('data.bin', 'rb') as f:
    data.fromfile(f, 100000)  # Direct binary I/O
```

### Functionality

**Lists**: Rich built-in API (append, extend, insert, remove, reverse, sort, etc.)

**arrayops**: Fast numeric operations (sum, mean, min, max, scale, clip, normalize, map, filter, reduce, sort, reverse, unique)

## Decision Matrix

| Requirement | Lists | arrayops |
|-------------|-------|----------|
| Mixed data types | ✅ | ❌ Numeric only |
| Small datasets (< 1K) | ✅ Faster | ⚠️ Overhead |
| Memory efficiency | ❌ High overhead | ✅ Excellent |
| Binary I/O | ❌ Manual | ✅ Native |
| C interop | ⚠️ Complex | ✅ Excellent |
| In-place operations | ⚠️ Limited | ✅ Excellent |
| Rich API | ✅ Comprehensive | ⚠️ Focused |

## Example: Same Task

**Task:** Process 1M sensor values, scale by 2, calculate mean

```python
# Lists
data = list(range(1_000_000))
scaled = [x * 2 for x in data]  # New list created
mean = sum(scaled) / len(scaled)

# arrayops
import array
import arrayops as ao
data = array.array('i', range(1_000_000))
ao.scale(data, 2.0)  # In-place
mean = ao.mean(data)  # Uses ~49% less memory
```

## Conclusion

**Lists** excel at flexibility, small datasets, and Python's built-in operations. **arrayops** excels at memory efficiency, binary I/O, and in-place numeric operations.

Choose lists for flexibility and small datasets. Choose arrayops for large numeric datasets when memory efficiency or binary I/O matter.
