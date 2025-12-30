# arrayops

Rust-backed acceleration for Python's `array.array` type. Fast, lightweight numeric operations without the overhead of NumPy.

## Overview

`arrayops` provides high-performance operations for Python's built-in `array.array` type by leveraging Rust and zero-copy buffer access. It fills the gap between Python's memory-efficient but slow `array.array` and the heavyweight NumPy library.

### Why arrayops?

- **Fast**: Rust-accelerated operations using zero-copy buffer access
- **Lightweight**: No dependencies beyond Rust standard library
- **Compatible**: Works directly with Python's `array.array` - no new types
- **Safe**: Memory-safe Rust implementation with proper error handling

## Installation

### Development Installation

```bash
# Install maturin if not already installed
pip install maturin

# Install in development mode
maturin develop
# or
pip install -e .
```

### Production Installation

```bash
# Build and install
maturin build --release
pip install target/wheels/arrayops-*.whl
```

## Supported Types

`arrayops` supports all numeric `array.array` typecodes:
- Signed integers: `b`, `h`, `i`, `l`
- Unsigned integers: `B`, `H`, `I`, `L`
- Floats: `f`, `d`

## API Reference

### `sum(arr) -> scalar`

Compute the sum of all elements in an array.

**Parameters:**
- `arr` (array.array): Input array (numeric types only)

**Returns:**
- Integer for integer arrays, float for float arrays

**Example:**
```python
import array
import arrayops

arr = array.array('i', [1, 2, 3, 4, 5])
result = arrayops.sum(arr)  # Returns 15
```

### `scale(arr, factor) -> None`

Scale all elements of an array in-place by a factor.

**Parameters:**
- `arr` (array.array): Input array (numeric types only)
- `factor` (float): Scaling factor

**Example:**
```python
import array
import arrayops

arr = array.array('i', [1, 2, 3, 4, 5])
arrayops.scale(arr, 2.0)
print(list(arr))  # [2, 4, 6, 8, 10]
```

## Examples

### Basic Usage

```python
import array
import arrayops

# Create an array
data = array.array('i', [10, 20, 30, 40, 50])

# Sum operation
total = arrayops.sum(data)
print(f"Sum: {total}")  # Sum: 150

# Scale operation (in-place)
arrayops.scale(data, 1.5)
print(list(data))  # [15, 30, 45, 60, 75]
```

### Binary Protocol Parsing

```python
import array
import arrayops

# Read binary data
with open('data.bin', 'rb') as f:
    data = array.array('i')
    data.fromfile(f, 1000)  # Read 1000 integers

# Fast aggregation
total = arrayops.sum(data)
average = total / len(data)
```

### ETL Pipeline

```python
import array
import arrayops

# Process sensor data
sensor_data = array.array('f', [...])  # Large float array

# Normalize data (scale to 0-1 range)
min_val = min(sensor_data)
max_val = max(sensor_data)
arrayops.scale(sensor_data, 1.0 / (max_val - min_val))

# Compute statistics
total = arrayops.sum(sensor_data)
mean = total / len(sensor_data)
```

## Performance

`arrayops` provides significant speedups over pure Python operations:

- **Sum**: 10-100x faster than Python iteration
- **Scale**: 5-50x faster than Python loops
- **Memory**: Zero-copy buffer access, no extra allocations

### Benchmark Example

```python
import array
import arrayops
import time

# Create large array
arr = array.array('i', list(range(1_000_000)))

# Python sum
start = time.time()
python_sum = sum(arr)
python_time = time.time() - start

# arrayops sum
start = time.time()
arrayops_sum = arrayops.sum(arr)
arrayops_time = time.time() - start

print(f"Python: {python_time:.4f}s")
print(f"arrayops: {arrayops_time:.4f}s")
print(f"Speedup: {python_time / arrayops_time:.1f}x")
```

## Comparison with NumPy

| Feature | array.array | arrayops | NumPy |
|---------|-------------|----------|-------|
| Memory efficient | ✅ | ✅ | ❌ |
| Fast operations | ❌ | ✅ | ✅ |
| Multi-dimensional | ❌ | ❌ | ✅ |
| Zero dependencies | ✅ | ✅ | ❌ |
| C-compatible | ✅ | ✅ | ✅ |
| Use case | Binary I/O | Scripting/ETL | Scientific computing |

## Development

### Building

```bash
# Install dependencies
pip install maturin pytest

# Build in debug mode
maturin develop

# Build in release mode
maturin build --release
```

### Running Tests

```bash
# Rust tests
cargo test

# Python tests
pytest tests/
```

### Optional Features

- `parallel`: Enable parallel execution with rayon (experimental)
- `simd`: Enable SIMD optimizations (future enhancement)

## Architecture

```
┌──────────────┐
│   Python     │
│              │
│ array.array  │  ← unchanged
│ arrayops.py  │
└──────┬───────┘
       │ buffer protocol
       ▼
┌──────────────┐
│    Rust      │
│  (PyO3)      │
│              │
│ typed loops  │
│ zero-copy    │
└──────────────┘
```

## Roadmap

- [ ] Additional operations (map, filter, reduce)
- [ ] Parallel execution support
- [ ] SIMD auto-vectorization
- [ ] NumPy array interop
- [ ] Memoryview support

## License

MIT License

## Contributing

Contributions welcome! Please see the design document for architecture details.

