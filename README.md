# arrayops

<div align="center">

**Rust-backed acceleration for Python's `array.array` type**

[![PyPI](https://img.shields.io/pypi/v/arrayops.svg)](https://pypi.org/project/arrayops/)
[![Python 3.8+](https://img.shields.io/badge/python-3.8+-blue.svg)](https://www.python.org/downloads/)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Documentation](https://readthedocs.org/projects/arrayops/badge/?version=latest)](https://arrayops.readthedocs.io/en/latest/?badge=latest)
[![Code Coverage](https://img.shields.io/badge/coverage-100%25-brightgreen.svg)](https://github.com/)

</div>

Fast, lightweight numeric operations for Python's `array.array`, `numpy.ndarray` (1D), and `memoryview` objects. Built with Rust and PyO3 for zero-copy, memory-safe performance.

## ✨ Features

- ⚡ **High Performance**: 10-100x faster than pure Python loops using Rust-accelerated operations
- 🔒 **Memory Safe**: Zero-copy buffer access with Rust's safety guarantees
- 📦 **Lightweight**: No dependencies beyond Rust standard library (optional: parallel execution via `rayon`)
- 🔌 **Compatible**: Works directly with Python's `array.array`, `numpy.ndarray` (1D), and `memoryview` - no new types
- ✅ **Fully Tested**: 100% code coverage (Python and Rust)
- 🎯 **Type Safe**: Full mypy type checking support
- 🚀 **Optional Optimizations**: Parallel execution and SIMD support via feature flags

## 🚀 Quick Start

### Installation

```bash
# Install maturin if not already installed
pip install maturin

# Install in development mode
maturin develop

# Or install from source
pip install -e .

# With optional features (recommended for large arrays)
maturin develop --features parallel
```

### Basic Usage

```python
import array
import arrayops as ao

# Create an array
data = array.array('i', [1, 2, 3, 4, 5])

# Fast operations
total = ao.sum(data)           # 15
ao.scale(data, 2.0)            # In-place: [2, 4, 6, 8, 10]
doubled = ao.map(data, lambda x: x * 2)  # New array: [4, 8, 12, 16, 20]
evens = ao.filter(data, lambda x: x % 2 == 0)  # [4, 8, 12, 16, 20]
product = ao.reduce(data, lambda acc, x: acc * x, initial=1)  # 3840
```

**📚 For complete documentation, examples, and API reference, see [arrayops.readthedocs.io](https://arrayops.readthedocs.io/)**

## 📚 Supported Types

`arrayops` supports all numeric `array.array` typecodes, `numpy.ndarray` (1D, contiguous), and Python `memoryview` objects:

| Type | Code | Description |
|------|------|-------------|
| Signed integers | `b`, `h`, `i`, `l` | int8, int16, int32, int64 |
| Unsigned integers | `B`, `H`, `I`, `L` | uint8, uint16, uint32, uint64 |
| Floats | `f`, `d` | float32, float64 |

## 📖 Documentation

Complete documentation is available at **[arrayops.readthedocs.io](https://arrayops.readthedocs.io/)**:

- **Getting Started** - Installation and basic usage
- **API Reference** - Complete function documentation
- **Examples** - Practical usage patterns and cookbook
- **Performance Guide** - Benchmark results and optimization tips
- **Troubleshooting** - Common issues and solutions

## ⚡ Performance

`arrayops` provides significant speedups over pure Python operations:

| Operation | Python | arrayops | Speedup |
|-----------|--------|----------|---------|
| Sum (1M ints) | ~50ms | ~0.5ms | 100x |
| Scale (1M ints) | ~80ms | ~1.5ms | 50x |
| Map (1M ints) | ~100ms | ~5ms | 20x |
| Filter (1M ints) | ~120ms | ~8ms | 15x |
| Reduce (1M ints) | ~150ms | ~6ms | 25x |
| Memory overhead | N/A | Zero-copy | — |

See the [Performance Guide](https://arrayops.readthedocs.io/en/latest/user-guide/performance.html) for detailed benchmarks and optimization tips.

### Performance Features

`arrayops` supports optional performance optimizations via feature flags:

#### Parallel Execution (`--features parallel`)

For large arrays, parallel execution can provide significant speedups on multi-core systems:

- **Enabled operations**: `sum`, `scale`
- **Threshold**: Arrays larger than 10,000 elements (sum) or 5,000 elements (scale) automatically use parallel processing
- **Installation**: `maturin develop --features parallel`
- **Performance**: 2-4x additional speedup on multi-core systems

#### SIMD Optimizations (`--features simd`)

SIMD (Single Instruction, Multiple Data) optimizations are in development:

- **Status**: Infrastructure in place, full implementation pending std::simd API stabilization
- **Expected performance**: 2-4x additional speedup on supported CPUs
- **Target operations**: `sum`, `scale` (primary), element-wise operations
- **Installation**: `maturin develop --features simd`

## 🔄 Comparison

| Feature | `array.array` | `arrayops` | NumPy |
|---------|---------------|------------|-------|
| Memory efficient | ✅ | ✅ | ❌ |
| Fast operations | ❌ | ✅ | ✅ |
| Multi-dimensional | ❌ | ❌ | ✅ |
| Zero dependencies | ✅ | ✅ (NumPy optional) | ❌ |
| C-compatible | ✅ | ✅ | ✅ |
| Type safety | ✅ | ✅ | ⚠️ |
| NumPy interop | ❌ | ✅ (1D only) | ✅ |
| Memoryview support | ❌ | ✅ | ❌ |
| Use case | Binary I/O | Scripting/ETL | Scientific computing |

## 🏗️ Architecture

```
┌─────────────────────────────────────────┐
│           Python Layer                  │
│  array.array → arrayops → _arrayops     │
└────────────────┬────────────────────────┘
                 │ Buffer Protocol
                 │ (Zero-copy)
                 ▼
┌─────────────────────────────────────────┐
│           Rust Layer (PyO3)             │
│  Typed operations                       │
│  SIMD / Parallel optimizations          │
└─────────────────────────────────────────┘
```

## 🧪 Testing

```bash
# Run all tests
pytest tests/ -v

# With coverage
pytest tests/ --cov=arrayops --cov-report=html

# Type checking
mypy arrayops tests
```

**Coverage**: 100% Python code coverage

## 🔧 Development

### Prerequisites

- Python 3.8+
- Rust 1.75+ (for SIMD features)
- `maturin` (install with `pip install maturin`)

### Building

```bash
# Development build
maturin develop

# Release build
maturin build --release

# With features
maturin develop --features parallel,simd
```

### Contributing

See the [Contributing Guide](https://arrayops.readthedocs.io/en/latest/developer-guide/contributing.html) for details on:

- Development workflow
- Code style guidelines
- Testing requirements
- Pull request process

## 📝 Error Handling

`arrayops` provides clear error messages:

```python
import arrayops as ao

# Wrong type
ao.sum([1, 2, 3])  # TypeError: Expected array.array, numpy.ndarray, or memoryview

# Unsupported typecode
arr = array.array('c', b'abc')
ao.sum(arr)  # TypeError: Unsupported typecode: 'c'
```

## 📄 License

MIT License - see LICENSE file for details.

## 🙏 Acknowledgments

- Built with [PyO3](https://pyo3.rs/) for Python-Rust interop
- Built with [maturin](https://github.com/PyO3/maturin) for packaging
- Inspired by the need for fast array operations without NumPy overhead

## 📞 Support

- **Documentation**: [arrayops.readthedocs.io](https://arrayops.readthedocs.io/)
- **Issues**: Report bugs or request features on [GitHub](https://github.com/eddiethedean/arrayops)
- **Questions**: Open a discussion on GitHub

---

<div align="center">

**For detailed documentation, examples, and API reference, visit [arrayops.readthedocs.io](https://arrayops.readthedocs.io/)**

</div>
