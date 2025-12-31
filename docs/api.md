# API Reference

Complete API documentation for the `arrayops` package.

## Overview

`arrayops` provides fast, Rust-accelerated operations for Python's built-in `array.array` type. All operations work directly with `array.array` objects without requiring new types or conversions.

## Supported Types

`arrayops` supports all numeric `array.array` typecodes:

| Type | Code | Description | Size |
|------|------|-------------|------|
| Signed integers | `b` | int8 | 1 byte |
| Signed integers | `h` | int16 | 2 bytes |
| Signed integers | `i` | int32 | 4 bytes |
| Signed integers | `l` | int64 | 8 bytes |
| Unsigned integers | `B` | uint8 | 1 byte |
| Unsigned integers | `H` | uint16 | 2 bytes |
| Unsigned integers | `I` | uint32 | 4 bytes |
| Unsigned integers | `L` | uint64 | 8 bytes |
| Floats | `f` | float32 | 4 bytes |
| Floats | `d` | float64 | 8 bytes |

## Functions

### `sum(arr) -> int | float`

Compute the sum of all elements in an array.

**Parameters:**
- `arr` (`array.array`): Input array with numeric type. Must be one of: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`

**Returns:**
- `int`: For integer arrays (`b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`)
- `float`: For float arrays (`f`, `d`)

**Raises:**
- `TypeError`: If input is not an `array.array` instance
- `TypeError`: If array uses an unsupported typecode

**Notes:**
- Empty arrays return `0` (integer) or `0.0` (float)
- Integer overflow follows Python's semantics (promotion to larger types)
- Performance: ~100x faster than Python's built-in `sum()` for large arrays

**Example:**
```python
import array
import arrayops

# Integer array
arr = array.array('i', [1, 2, 3, 4, 5])
result = arrayops.sum(arr)
print(result)  # 15
print(type(result))  # <class 'int'>

# Float array
farr = array.array('f', [1.5, 2.5, 3.5])
result = arrayops.sum(farr)
print(result)  # 7.5
print(type(result))  # <class 'float'>

# Empty array
empty = array.array('i', [])
result = arrayops.sum(empty)
print(result)  # 0
```

**See also:**
- [Examples: Basic Operations](examples.md#basic-operations)
- [Performance Guide](performance.md#sum-operation)

---

### `scale(arr, factor) -> None`

Scale all elements of an array in-place by a factor.

**Parameters:**
- `arr` (`array.array`): Input array with numeric type (modified in-place). Must be one of: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`
- `factor` (`float`): Scaling factor to multiply each element by

**Returns:**
- `None`: This function modifies the array in-place and returns nothing

**Raises:**
- `TypeError`: If input is not an `array.array` instance
- `TypeError`: If array uses an unsupported typecode

**Notes:**
- The array is modified in-place; no new array is created
- For integer arrays, the factor is cast to the array's integer type
- Empty arrays are handled gracefully (no error, array remains empty)
- Performance: ~50x faster than Python loops for large arrays

**Example:**
```python
import array
import arrayops

# Integer array
arr = array.array('i', [1, 2, 3, 4, 5])
arrayops.scale(arr, 2.0)
print(list(arr))  # [2, 4, 6, 8, 10]

# Float array
farr = array.array('f', [1.0, 2.0, 3.0])
arrayops.scale(farr, 1.5)
print(list(farr))  # [1.5, 3.0, 4.5]

# Negative factor
arr = array.array('i', [1, 2, 3])
arrayops.scale(arr, -1.0)
print(list(arr))  # [-1, -2, -3]

# Zero factor
arr = array.array('i', [1, 2, 3])
arrayops.scale(arr, 0.0)
print(list(arr))  # [0, 0, 0]
```

**See also:**
- [Examples: ETL Pipeline](examples.md#etl-pipeline)
- [Performance Guide](performance.md#scale-operation)

---

## Error Handling

All functions provide clear, descriptive error messages:

```python
import array
import arrayops

# Wrong type
try:
    arrayops.sum([1, 2, 3])  # TypeError: Expected array.array
except TypeError as e:
    print(e)

# Unsupported typecode
try:
    arr = array.array('c', b'abc')  # Character array
    arrayops.sum(arr)  # TypeError: Unsupported typecode: 'c'
except TypeError as e:
    print(e)
```

## Type Safety

All functions validate their inputs:
- Type checking ensures only `array.array` instances are accepted
- Typecode validation ensures only numeric types are supported
- Clear error messages guide users to correct usage

For static type checking with mypy, type stubs are provided in `arrayops._arrayops`.

## Performance Characteristics

| Operation | Python | arrayops | Speedup |
|-----------|--------|----------|---------|
| Sum (1M ints) | ~50ms | ~0.5ms | 100x |
| Scale (1M ints) | ~80ms | ~1.5ms | 50x |
| Memory overhead | N/A | Zero-copy | — |

See the [Performance Guide](performance.md) for detailed benchmarking and optimization tips.

## Zero-Copy Operations

All operations use Python's buffer protocol for zero-copy access:
- No data copying between Python and Rust
- Direct memory access to array data
- Memory-safe operations guaranteed by Rust

## Related Documentation

- [Examples and Cookbook](examples.md) - Practical usage examples
- [Performance Guide](performance.md) - Performance analysis and optimization
- [Design Document](design.md) - Architecture and implementation details
- [Troubleshooting](troubleshooting.md) - Common issues and solutions

