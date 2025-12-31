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

### `map(arr, fn) -> array.array`

Apply a function to each element, returning a new array.

**Parameters:**
- `arr` (`array.array`): Input array with numeric type. Must be one of: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`
- `fn` (callable): Function that takes one element and returns a value of the same type as the input array

**Returns:**
- `array.array`: New array with the same type as the input array

**Raises:**
- `TypeError`: If input is not an `array.array` instance
- `TypeError`: If array uses an unsupported typecode
- `TypeError`: If `fn` is not callable
- `TypeError`: If function returns a value that cannot be converted to the array's type

**Notes:**
- Returns a new array; the original array is not modified
- The function must return a value that can be converted to the array's element type
- Empty arrays return an empty array of the same type
- Performance: ~20x faster than Python list comprehensions for large arrays

**Example:**
```python
import array
import arrayops

# Double each element
arr = array.array('i', [1, 2, 3, 4, 5])
doubled = arrayops.map(arr, lambda x: x * 2)
print(list(doubled))  # [2, 4, 6, 8, 10]

# Using named function
def square(x):
    return x * x

squared = arrayops.map(arr, square)
print(list(squared))  # [1, 4, 9, 16, 25]

# Float arrays
farr = array.array('f', [1.5, 2.5, 3.5])
halved = arrayops.map(farr, lambda x: x / 2.0)
print(list(halved))  # [0.75, 1.25, 1.75]
```

**See also:**
- [Examples: Data Transformation](examples.md#data-transformation)
- [Performance Guide](performance.md)

---

### `map_inplace(arr, fn) -> None`

Apply a function to each element in-place.

**Parameters:**
- `arr` (`array.array`): Input array with numeric type (modified in-place). Must be one of: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`
- `fn` (callable): Function that takes one element and returns a value of the same type as the input array

**Returns:**
- `None`: This function modifies the array in-place and returns nothing

**Raises:**
- `TypeError`: If input is not an `array.array` instance
- `TypeError`: If array uses an unsupported typecode
- `TypeError`: If `fn` is not callable
- `TypeError`: If function returns a value that cannot be converted to the array's type

**Notes:**
- The array is modified in-place; no new array is created
- The function must return a value that can be converted to the array's element type
- Empty arrays are handled gracefully (no error, array remains empty)
- Performance: ~15x faster than Python loops for large arrays

**Example:**
```python
import array
import arrayops

# Double each element in-place
arr = array.array('i', [1, 2, 3, 4, 5])
arrayops.map_inplace(arr, lambda x: x * 2)
print(list(arr))  # [2, 4, 6, 8, 10]

# Square in-place
def square(x):
    return x * x

arrayops.map_inplace(arr, square)
print(list(arr))  # [4, 16, 36, 64, 100]
```

**See also:**
- [Examples: Data Transformation](examples.md#data-transformation)
- [Performance Guide](performance.md)

---

### `filter(arr, predicate) -> array.array`

Filter elements using a predicate function, returning a new array.

**Parameters:**
- `arr` (`array.array`): Input array with numeric type. Must be one of: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`
- `predicate` (callable): Function that takes one element and returns `bool`

**Returns:**
- `array.array`: New array containing only elements where `predicate(element)` is `True` (same type as input)

**Raises:**
- `TypeError`: If input is not an `array.array` instance
- `TypeError`: If array uses an unsupported typecode
- `TypeError`: If `predicate` is not callable
- `TypeError`: If predicate doesn't return `bool`

**Notes:**
- Returns a new array; the original array is not modified
- The predicate function must return a boolean value (use `bool()` explicitly if needed)
- Empty arrays return an empty array of the same type
- Performance: ~15x faster than Python list comprehensions for large arrays

**Example:**
```python
import array
import arrayops

# Filter even numbers
arr = array.array('i', [1, 2, 3, 4, 5, 6])
evens = arrayops.filter(arr, lambda x: x % 2 == 0)
print(list(evens))  # [2, 4, 6]

# Filter values greater than threshold
large = arrayops.filter(arr, lambda x: x > 3)
print(list(large))  # [4, 5, 6]

# Filter with named function
def is_positive(x):
    return x > 0

arr_neg = array.array('i', [-2, -1, 0, 1, 2])
positives = arrayops.filter(arr_neg, is_positive)
print(list(positives))  # [1, 2]
```

**See also:**
- [Examples: Data Filtering](examples.md#data-filtering)
- [Performance Guide](performance.md)

---

### `reduce(arr, fn, initial=None) -> Any`

Reduce array to a single value using a binary function.

**Parameters:**
- `arr` (`array.array`): Input array with numeric type. Must be one of: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`
- `fn` (callable): Binary function that takes `(accumulator, element)` and returns a value
- `initial` (optional): Initial value for the accumulator. If not provided, uses the first element as initial value.

**Returns:**
- Any: Result of the reduction (type depends on function and initial value)

**Raises:**
- `TypeError`: If input is not an `array.array` instance
- `TypeError`: If array uses an unsupported typecode
- `TypeError`: If `fn` is not callable
- `ValueError`: If array is empty and no initial value is provided

**Notes:**
- If `initial` is provided, reduction starts with that value and processes all elements
- If `initial` is not provided, the first element is used as the initial value and processing starts from the second element
- Empty arrays require an `initial` value
- The return type can be different from the array element type (e.g., reducing to a string)
- Performance: ~25x faster than Python's `functools.reduce` for large arrays

**Example:**
```python
import array
import arrayops

arr = array.array('i', [1, 2, 3, 4, 5])

# Sum using reduce
total = arrayops.reduce(arr, lambda acc, x: acc + x)
print(total)  # 15

# Product with initial value
product = arrayops.reduce(arr, lambda acc, x: acc * x, initial=1)
print(product)  # 120

# Maximum value (no initial)
maximum = arrayops.reduce(arr, lambda acc, x: acc if acc > x else x)
print(maximum)  # 5

# Minimum with initial
minimum = arrayops.reduce(arr, lambda acc, x: acc if acc < x else x, initial=100)
print(minimum)  # 1

# Reduce to string (different return type)
result = arrayops.reduce(arr, lambda acc, x: f"{acc}+{x}", initial="0")
print(result)  # "0+1+2+3+4+5"

# Empty array requires initial
empty = array.array('i', [])
total = arrayops.reduce(empty, lambda acc, x: acc + x, initial=0)
print(total)  # 0
```

**See also:**
- [Examples: Aggregation](examples.md#aggregation)
- [Performance Guide](performance.md)

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

# Non-callable function
try:
    arr = array.array('i', [1, 2, 3])
    arrayops.map(arr, "not a function")  # TypeError: Expected callable
except TypeError as e:
    print(e)

# Empty array with reduce (no initial)
try:
    empty = array.array('i', [])
    arrayops.reduce(empty, lambda acc, x: acc + x)  # ValueError: reduce() of empty array
except ValueError as e:
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
| Map (1M ints) | ~100ms | ~5ms | 20x |
| Filter (1M ints) | ~120ms | ~8ms | 15x |
| Reduce (1M ints) | ~150ms | ~6ms | 25x |
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

