# Getting Started

Welcome to `arrayops`! This guide will help you get started quickly.

## What is arrayops?

`arrayops` is a high-performance library for Python's built-in `array.array` type, providing Rust-accelerated operations that are 10-100x faster than pure Python implementations.

## Installation

### Prerequisites

- Python 3.8 or higher
- Rust 1.75 or higher (for building from source)
- `maturin` (install with `pip install maturin`)

### Basic Installation

```bash
# Install maturin if not already installed
pip install maturin

# Install in development mode
maturin develop

# Or install from source
pip install -e .
```

### Installation with Optional Features

```bash
# Install with parallel execution support (recommended for large arrays)
maturin develop --features parallel

# Install with SIMD optimizations (infrastructure in place, full implementation pending)
maturin develop --features simd

# Install with both features
maturin develop --features parallel,simd

# For production wheels
maturin build --release --features parallel
```

## Quick Example

```python
import array
import arrayops

# Create an array
data = array.array('i', [1, 2, 3, 4, 5])

# Fast sum operation
total = arrayops.sum(data)
print(total)  # 15

# In-place scaling
arrayops.scale(data, 2.0)
print(list(data))  # [2, 4, 6, 8, 10]

# Map operation (returns new array)
doubled = arrayops.map(data, lambda x: x * 2)
print(list(doubled))  # [4, 8, 12, 16, 20]

# Filter operation
evens = arrayops.filter(data, lambda x: x % 2 == 0)
print(list(evens))  # [2, 4, 6, 8, 10]

# Reduce operation
product = arrayops.reduce(data, lambda acc, x: acc * x, initial=1)
print(product)  # 120
```

## Supported Types

`arrayops` works with:

- **`array.array`** - Python's built-in array type
- **`numpy.ndarray`** - NumPy arrays (1D, contiguous only)
- **`memoryview`** - Python memoryview objects

All numeric types are supported: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`

## Next Steps

- Read the [API Reference](../user-guide/api.md) for complete function documentation
- Check out [Examples](../user-guide/examples.md) for practical usage patterns
- Learn about [Performance](../user-guide/performance.md) optimization
- See [Troubleshooting](../user-guide/troubleshooting.md) if you encounter issues

## Key Features

- ⚡ **High Performance**: 10-100x faster than pure Python loops
- 🔒 **Memory Safe**: Zero-copy buffer access with Rust's safety guarantees
- 📦 **Lightweight**: No dependencies beyond Rust standard library
- 🔌 **Compatible**: Works directly with existing array types - no new types
- ✅ **Fully Tested**: 100% code coverage
- 🎯 **Type Safe**: Full mypy type checking support

## When to Use arrayops

`arrayops` is ideal for:

- Processing large numeric arrays efficiently
- Binary protocol parsing and data processing
- ETL pipelines with array operations
- Performance-critical code using `array.array`
- Scenarios where NumPy's overhead is too much

`arrayops` is NOT ideal for:

- Multi-dimensional arrays (use NumPy)
- Scientific computing with complex operations (use NumPy)
- When you need a full array library (use NumPy)

## Getting Help

- **Documentation**: Browse the [User Guide](../user-guide/api.md)
- **Examples**: See [Examples](../user-guide/examples.md)
- **Issues**: Report bugs on [GitHub](https://github.com/eddiethedean/arrayops)
- **Questions**: Open a discussion on GitHub

