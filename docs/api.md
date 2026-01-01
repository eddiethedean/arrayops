# API Reference

Complete API documentation for the `arrayops` package.

## Overview

`arrayops` provides fast, Rust-accelerated operations for Python's built-in `array.array` type, `numpy.ndarray` (1D arrays), and Python `memoryview` objects. All operations work directly with these types without requiring new types or conversions.

### Security Considerations

All arrayops functions perform comprehensive input validation and use Rust's memory safety guarantees. For detailed security information, see the [Security Documentation](../security.md). Key security points:

- **Input validation**: All inputs are validated (type, typecode, properties) before processing
- **Memory safety**: Rust's compile-time memory safety prevents buffer overflows and use-after-free
- **Error handling**: Errors are handled gracefully without exposing sensitive information
- **No unsafe patterns**: All buffer access uses safe PyO3 APIs

When using arrayops with untrusted input, consider implementing size limits to prevent denial of service (DoS) attacks through resource exhaustion.

## Supported Types

`arrayops` supports all numeric `array.array` typecodes, `numpy.ndarray` (1D, contiguous), and `memoryview` objects:

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
- `arr` (`array.array`, `numpy.ndarray`, or `memoryview`): Input array with numeric type. Must be one of: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`
  - For `numpy.ndarray`: must be 1-dimensional and contiguous (C_CONTIGUOUS or F_CONTIGUOUS)
  - For `memoryview`: read-only or writable memoryview objects are supported

**Returns:**
- `int`: For integer arrays (`b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`)
- `float`: For float arrays (`f`, `d`)

**Raises:**
- `TypeError`: If input is not an `array.array`, `numpy.ndarray`, or `memoryview`
- `TypeError`: If array uses an unsupported typecode
- `TypeError`: If `numpy.ndarray` is not 1D or not contiguous

**Notes:**
- Empty arrays return `0` (integer) or `0.0` (float)
- Integer overflow follows Python's semantics (promotion to larger types)
- Performance: ~100x faster than Python's built-in `sum()` for large arrays
- Parallel execution: When built with `--features parallel`, arrays with 10,000+ elements automatically use parallel processing for additional speedup on multi-core systems
- SIMD optimization: Infrastructure available via `--features simd` (full implementation pending)

**Example:**
```python
import array
import arrayops as ao

# Integer array
arr = array.array('i', [1, 2, 3, 4, 5])
result = ao.sum(arr)
print(result)  # 15
print(type(result))  # <class 'int'>

# Float array
farr = array.array('f', [1.5, 2.5, 3.5])
result = ao.sum(farr)
print(result)  # 7.5
print(type(result))  # <class 'float'>

# Empty array
empty = array.array('i', [])
result = ao.sum(empty)
print(result)  # 0
```

**See also:**
- [Examples: Basic Operations](examples.md#basic-operations)
- [Performance Guide](performance.md#sum-operation)

---

### `scale(arr, factor) -> None`

Scale all elements of an array in-place by a factor.

**Parameters:**
- `arr` (`array.array`, `numpy.ndarray`, or `memoryview`): Input array with numeric type (modified in-place). Must be one of: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`
  - For `numpy.ndarray`: must be 1D and contiguous
  - For `memoryview`: must be writable (read-only memoryviews raise ValueError)
- `factor` (`float`): Scaling factor to multiply each element by

**Returns:**
- `None`: This function modifies the array in-place and returns nothing

**Raises:**
- `TypeError`: If input is not an `array.array`, `numpy.ndarray`, or `memoryview`
- `TypeError`: If array uses an unsupported typecode
- `TypeError`: If `numpy.ndarray` is not 1D or not contiguous
- `ValueError`: If `memoryview` is read-only

**Notes:**
- The array is modified in-place; no new array is created
- For integer arrays, the factor is cast to the array's integer type
- Empty arrays are handled gracefully (no error, array remains empty)
- Performance: ~50x faster than Python loops for large arrays
- Parallel execution: When built with `--features parallel`, arrays with 5,000+ elements automatically use parallel processing for additional speedup on multi-core systems
- SIMD optimization: Infrastructure available via `--features simd` (full implementation pending)

**Example:**
```python
import array
import arrayops as ao

# Integer array
arr = array.array('i', [1, 2, 3, 4, 5])
ao.scale(arr, 2.0)
print(list(arr))  # [2, 4, 6, 8, 10]

# Float array
farr = array.array('f', [1.0, 2.0, 3.0])
ao.scale(farr, 1.5)
print(list(farr))  # [1.5, 3.0, 4.5]

# Negative factor
arr = array.array('i', [1, 2, 3])
ao.scale(arr, -1.0)
print(list(arr))  # [-1, -2, -3]

# Zero factor
arr = array.array('i', [1, 2, 3])
ao.scale(arr, 0.0)
print(list(arr))  # [0, 0, 0]
```

**See also:**
- [Examples: ETL Pipeline](examples.md#etl-pipeline)
- [Performance Guide](performance.md#scale-operation)

---

### `map(arr, fn) -> array.array | numpy.ndarray`

Apply a function to each element, returning a new array.

**Parameters:**
- `arr` (`array.array`, `numpy.ndarray`, or `memoryview`): Input array with numeric type. Must be one of: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`
- `fn` (callable): Function that takes one element and returns a value of the same type as the input array

**Returns:**
- `array.array` or `numpy.ndarray`: New array with the same type as the input array
  - Returns `numpy.ndarray` if input is `numpy.ndarray`
  - Returns `array.array` if input is `array.array` or `memoryview`

**Raises:**
- `TypeError`: If input is not an `array.array`, `numpy.ndarray`, or `memoryview`
- `TypeError`: If array uses an unsupported typecode
- `TypeError`: If `numpy.ndarray` is not 1D or not contiguous
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
import arrayops as ao

# Double each element
arr = array.array('i', [1, 2, 3, 4, 5])
doubled = ao.map(arr, lambda x: x * 2)
print(list(doubled))  # [2, 4, 6, 8, 10]

# Using named function
def square(x):
    return x * x

squared = ao.map(arr, square)
print(list(squared))  # [1, 4, 9, 16, 25]

# Float arrays
farr = array.array('f', [1.5, 2.5, 3.5])
halved = ao.map(farr, lambda x: x / 2.0)
print(list(halved))  # [0.75, 1.25, 1.75]
```

**See also:**
- [Examples: Data Transformation](examples.md#data-transformation)
- [Performance Guide](performance.md)

---

### `map_inplace(arr, fn) -> None`

Apply a function to each element in-place.

**Parameters:**
- `arr` (`array.array`, `numpy.ndarray`, or `memoryview`): Input array with numeric type (modified in-place). Must be one of: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`
  - For `memoryview`: must be writable
- `fn` (callable): Function that takes one element and returns a value of the same type as the input array

**Returns:**
- `None`: This function modifies the array in-place and returns nothing

**Raises:**
- `TypeError`: If input is not an `array.array`, `numpy.ndarray`, or `memoryview`
- `TypeError`: If array uses an unsupported typecode
- `TypeError`: If `numpy.ndarray` is not 1D or not contiguous
- `TypeError`: If `fn` is not callable
- `TypeError`: If function returns a value that cannot be converted to the array's type
- `ValueError`: If `memoryview` is read-only

**Notes:**
- The array is modified in-place; no new array is created
- The function must return a value that can be converted to the array's element type
- Empty arrays are handled gracefully (no error, array remains empty)
- Performance: ~15x faster than Python loops for large arrays

**Example:**
```python
import array
import arrayops as ao

# Double each element in-place
arr = array.array('i', [1, 2, 3, 4, 5])
ao.map_inplace(arr, lambda x: x * 2)
print(list(arr))  # [2, 4, 6, 8, 10]

# Square in-place
def square(x):
    return x * x

ao.map_inplace(arr, square)
print(list(arr))  # [4, 16, 36, 64, 100]
```

**See also:**
- [Examples: Data Transformation](examples.md#data-transformation)
- [Performance Guide](performance.md)

---

### `filter(arr, predicate) -> array.array | numpy.ndarray`

Filter elements using a predicate function, returning a new array.

**Parameters:**
- `arr` (`array.array`, `numpy.ndarray`, or `memoryview`): Input array with numeric type. Must be one of: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`
- `predicate` (callable): Function that takes one element and returns `bool`

**Returns:**
- `array.array` or `numpy.ndarray`: New array containing only elements where `predicate(element)` is `True`
  - Returns `numpy.ndarray` if input is `numpy.ndarray`
  - Returns `array.array` if input is `array.array` or `memoryview`

**Raises:**
- `TypeError`: If input is not an `array.array`, `numpy.ndarray`, or `memoryview`
- `TypeError`: If array uses an unsupported typecode
- `TypeError`: If `numpy.ndarray` is not 1D or not contiguous
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
import arrayops as ao

# Filter even numbers
arr = array.array('i', [1, 2, 3, 4, 5, 6])
evens = ao.filter(arr, lambda x: x % 2 == 0)
print(list(evens))  # [2, 4, 6]

# Filter values greater than threshold
large = ao.filter(arr, lambda x: x > 3)
print(list(large))  # [4, 5, 6]

# Filter with named function
def is_positive(x):
    return x > 0

arr_neg = array.array('i', [-2, -1, 0, 1, 2])
positives = ao.filter(arr_neg, is_positive)
print(list(positives))  # [1, 2]
```

**See also:**
- [Examples: Data Filtering](examples.md#data-filtering)
- [Performance Guide](performance.md)

---

### `reduce(arr, fn, initial=None) -> Any`

Reduce array to a single value using a binary function.

**Parameters:**
- `arr` (`array.array`, `numpy.ndarray`, or `memoryview`): Input array with numeric type. Must be one of: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`
- `fn` (callable): Binary function that takes `(accumulator, element)` and returns a value
- `initial` (optional): Initial value for the accumulator. If not provided, uses the first element as initial value.

**Returns:**
- Any: Result of the reduction (type depends on function and initial value)

**Raises:**
- `TypeError`: If input is not an `array.array`, `numpy.ndarray`, or `memoryview`
- `TypeError`: If array uses an unsupported typecode
- `TypeError`: If `numpy.ndarray` is not 1D or not contiguous
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
import arrayops as ao

arr = array.array('i', [1, 2, 3, 4, 5])

# Sum using reduce
total = ao.reduce(arr, lambda acc, x: acc + x)
print(total)  # 15

# Product with initial value
product = ao.reduce(arr, lambda acc, x: acc * x, initial=1)
print(product)  # 120

# Maximum value (no initial)
maximum = ao.reduce(arr, lambda acc, x: acc if acc > x else x)
print(maximum)  # 5

# Minimum with initial
minimum = ao.reduce(arr, lambda acc, x: acc if acc < x else x, initial=100)
print(minimum)  # 1

# Reduce to string (different return type)
result = ao.reduce(arr, lambda acc, x: f"{acc}+{x}", initial="0")
print(result)  # "0+1+2+3+4+5"

# Empty array requires initial
empty = array.array('i', [])
total = ao.reduce(empty, lambda acc, x: acc + x, initial=0)
print(total)  # 0
```

**See also:**
- [Examples: Aggregation](examples.md#aggregation)
- [Performance Guide](performance.md)

---

## Statistical Operations

### `mean(arr) -> float`

Compute the arithmetic mean (average) of all elements in an array.

**Parameters:**
- `arr` (`array.array`, `numpy.ndarray`, or `memoryview`): Input array with numeric type. Must be one of: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`
  - For `numpy.ndarray`: must be 1-dimensional and contiguous (C_CONTIGUOUS or F_CONTIGUOUS)
  - For `memoryview`: read-only or writable memoryview objects are supported

**Returns:**
- `float`: Arithmetic mean of array elements

**Raises:**
- `TypeError`: If input is not an `array.array`, `numpy.ndarray`, or `memoryview`
- `TypeError`: If array uses an unsupported typecode
- `TypeError`: If `numpy.ndarray` is not 1D or not contiguous
- `ValueError`: If array is empty

**Notes:**
- Always returns a `float`, even for integer arrays
- Empty arrays raise `ValueError`
- Performance: ~50x faster than computing mean in pure Python

**Example:**
```python
import array
import arrayops as ao

arr = array.array('i', [1, 2, 3, 4, 5])
avg = ao.mean(arr)
print(avg)  # 3.0

float_arr = array.array('f', [1.5, 2.5, 3.5, 4.5])
avg_float = ao.mean(float_arr)
print(avg_float)  # 3.0
```

---

### `min(arr) -> int | float`

Find the minimum value in an array.

**Parameters:**
- `arr` (`array.array`, `numpy.ndarray`, or `memoryview`): Input array with numeric type. Must be one of: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`
  - For `numpy.ndarray`: must be 1-dimensional and contiguous (C_CONTIGUOUS or F_CONTIGUOUS)
  - For `memoryview`: read-only or writable memoryview objects are supported

**Returns:**
- `int`: For integer arrays (`b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`)
- `float`: For float arrays (`f`, `d`)

**Raises:**
- `TypeError`: If input is not an `array.array`, `numpy.ndarray`, or `memoryview`
- `TypeError`: If array uses an unsupported typecode
- `TypeError`: If `numpy.ndarray` is not 1D or not contiguous
- `ValueError`: If array is empty

**Notes:**
- Returns type matches array element type
- Empty arrays raise `ValueError`
- Performance: ~30x faster than Python's built-in `min()` for large arrays

**Example:**
```python
import array
import arrayops as ao

arr = array.array('i', [5, 2, 8, 1, 9])
minimum = ao.min(arr)
print(minimum)  # 1

float_arr = array.array('f', [3.5, 1.2, 7.8, 0.5])
minimum_float = ao.min(float_arr)
print(minimum_float)  # 0.5
```

---

### `max(arr) -> int | float`

Find the maximum value in an array.

**Parameters:**
- `arr` (`array.array`, `numpy.ndarray`, or `memoryview`): Input array with numeric type. Must be one of: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`
  - For `numpy.ndarray`: must be 1-dimensional and contiguous (C_CONTIGUOUS or F_CONTIGUOUS)
  - For `memoryview`: read-only or writable memoryview objects are supported

**Returns:**
- `int`: For integer arrays (`b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`)
- `float`: For float arrays (`f`, `d`)

**Raises:**
- `TypeError`: If input is not an `array.array`, `numpy.ndarray`, or `memoryview`
- `TypeError`: If array uses an unsupported typecode
- `TypeError`: If `numpy.ndarray` is not 1D or not contiguous
- `ValueError`: If array is empty

**Notes:**
- Returns type matches array element type
- Empty arrays raise `ValueError`
- Performance: ~30x faster than Python's built-in `max()` for large arrays

**Example:**
```python
import array
import arrayops as ao

arr = array.array('i', [5, 2, 8, 1, 9])
maximum = ao.max(arr)
print(maximum)  # 9

float_arr = array.array('f', [3.5, 1.2, 7.8, 0.5])
maximum_float = ao.max(float_arr)
print(maximum_float)  # 7.8
```

---

### `std(arr) -> float`

Compute the population standard deviation of array elements.

**Parameters:**
- `arr` (`array.array`, `numpy.ndarray`, or `memoryview`): Input array with numeric type. Must be one of: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`
  - For `numpy.ndarray`: must be 1-dimensional and contiguous (C_CONTIGUOUS or F_CONTIGUOUS)
  - For `memoryview`: read-only or writable memoryview objects are supported

**Returns:**
- `float`: Population standard deviation (sqrt of variance)

**Raises:**
- `TypeError`: If input is not an `array.array`, `numpy.ndarray`, or `memoryview`
- `TypeError`: If array uses an unsupported typecode
- `TypeError`: If `numpy.ndarray` is not 1D or not contiguous
- `ValueError`: If array is empty

**Notes:**
- Uses population standard deviation: `sqrt(sum((x - mean)^2) / n)`
- Always returns a `float`
- Empty arrays raise `ValueError`
- Performance: ~40x faster than computing std in pure Python

**Example:**
```python
import array
import arrayops as ao

arr = array.array('i', [1, 2, 3, 4, 5])
std_dev = ao.std(arr)
print(std_dev)  # Approximately 1.414 (sqrt of 2.0)
```

---

### `var(arr) -> float`

Compute the population variance of array elements.

**Parameters:**
- `arr` (`array.array`, `numpy.ndarray`, or `memoryview`): Input array with numeric type. Must be one of: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`
  - For `numpy.ndarray`: must be 1-dimensional and contiguous (C_CONTIGUOUS or F_CONTIGUOUS)
  - For `memoryview`: read-only or writable memoryview objects are supported

**Returns:**
- `float`: Population variance

**Raises:**
- `TypeError`: If input is not an `array.array`, `numpy.ndarray`, or `memoryview`
- `TypeError`: If array uses an unsupported typecode
- `TypeError`: If `numpy.ndarray` is not 1D or not contiguous
- `ValueError`: If array is empty

**Notes:**
- Uses population variance: `sum((x - mean)^2) / n`
- Always returns a `float`
- Empty arrays raise `ValueError`
- Performance: ~40x faster than computing variance in pure Python

**Example:**
```python
import array
import arrayops as ao

arr = array.array('i', [1, 2, 3, 4, 5])
variance = ao.var(arr)
print(variance)  # 2.0
```

---

### `median(arr) -> int | float`

Find the median value in an array.

**Parameters:**
- `arr` (`array.array`, `numpy.ndarray`, or `memoryview`): Input array with numeric type. Must be one of: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`
  - For `numpy.ndarray`: must be 1-dimensional and contiguous (C_CONTIGUOUS or F_CONTIGUOUS)
  - For `memoryview`: read-only or writable memoryview objects are supported

**Returns:**
- `int`: For integer arrays (`b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`)
- `float`: For float arrays (`f`, `d`)

**Raises:**
- `TypeError`: If input is not an `array.array`, `numpy.ndarray`, or `memoryview`
- `TypeError`: If array uses an unsupported typecode
- `TypeError`: If `numpy.ndarray` is not 1D or not contiguous
- `ValueError`: If array is empty

**Notes:**
- For odd-length arrays, returns the middle element
- For even-length arrays, returns the lower median (element at index `(n-1)/2` after sorting)
- Returns type matches array element type
- Empty arrays raise `ValueError`
- Performance: ~20x faster than computing median in pure Python

**Example:**
```python
import array
import arrayops as ao

# Odd-length array
arr1 = array.array('i', [5, 2, 8, 1, 9])
median1 = ao.median(arr1)
print(median1)  # 5

# Even-length array (returns lower median)
arr2 = array.array('i', [1, 2, 5, 8])
median2 = ao.median(arr2)
print(median2)  # 2
```

---

## Element-wise Operations

### `add(arr1, arr2) -> array.array | numpy.ndarray`

Element-wise addition of two arrays.

**Parameters:**
- `arr1` (`array.array`, `numpy.ndarray`, or `memoryview`): First input array
- `arr2` (`array.array`, `numpy.ndarray`, or `memoryview`): Second input array
  - Both arrays must have the same length
  - Both arrays must have compatible numeric types

**Returns:**
- `array.array` or `numpy.ndarray`: New array containing element-wise sums
  - Returns `numpy.ndarray` if both inputs are `numpy.ndarray`
  - Returns `array.array` if inputs are `array.array` or `memoryview`
  - Result type matches input types

**Raises:**
- `TypeError`: If inputs are not `array.array`, `numpy.ndarray`, or `memoryview`
- `TypeError`: If arrays use unsupported typecodes
- `TypeError`: If `numpy.ndarray` inputs are not 1D or not contiguous
- `ValueError`: If arrays have different lengths

**Notes:**
- Returns a new array; input arrays are not modified
- Arrays must have the same length
- Performance: ~30x faster than Python loops for large arrays

**Example:**
```python
import array
import arrayops as ao

arr1 = array.array('i', [1, 2, 3, 4, 5])
arr2 = array.array('i', [10, 20, 30, 40, 50])
result = ao.add(arr1, arr2)
print(list(result))  # [11, 22, 33, 44, 55]
```

---

### `multiply(arr1, arr2) -> array.array | numpy.ndarray`

Element-wise multiplication of two arrays.

**Parameters:**
- `arr1` (`array.array`, `numpy.ndarray`, or `memoryview`): First input array
- `arr2` (`array.array`, `numpy.ndarray`, or `memoryview`): Second input array
  - Both arrays must have the same length
  - Both arrays must have compatible numeric types

**Returns:**
- `array.array` or `numpy.ndarray`: New array containing element-wise products
  - Returns `numpy.ndarray` if both inputs are `numpy.ndarray`
  - Returns `array.array` if inputs are `array.array` or `memoryview`
  - Result type matches input types

**Raises:**
- `TypeError`: If inputs are not `array.array`, `numpy.ndarray`, or `memoryview`
- `TypeError`: If arrays use unsupported typecodes
- `TypeError`: If `numpy.ndarray` inputs are not 1D or not contiguous
- `ValueError`: If arrays have different lengths

**Notes:**
- Returns a new array; input arrays are not modified
- Arrays must have the same length
- Performance: ~30x faster than Python loops for large arrays

**Example:**
```python
import array
import arrayops as ao

arr1 = array.array('i', [1, 2, 3, 4, 5])
arr2 = array.array('i', [2, 3, 4, 5, 6])
result = ao.multiply(arr1, arr2)
print(list(result))  # [2, 6, 12, 20, 30]
```

---

### `clip(arr, min_val, max_val) -> None`

Clip array elements to be within the specified range [min_val, max_val] in-place.

**Parameters:**
- `arr` (`array.array`, `numpy.ndarray`, or `memoryview`): Input array with numeric type. Must be one of: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`
  - For `numpy.ndarray`: must be 1-dimensional and contiguous (C_CONTIGUOUS or F_CONTIGUOUS)
  - For `memoryview`: must be writable (in-place operations require writable memoryview)
- `min_val` (`float`): Minimum value (elements below this are set to min_val)
- `max_val` (`float`): Maximum value (elements above this are set to max_val)

**Returns:**
- `None`: Array is modified in-place

**Raises:**
- `TypeError`: If input is not an `array.array`, `numpy.ndarray`, or `memoryview`
- `TypeError`: If array uses an unsupported typecode
- `TypeError`: If `numpy.ndarray` is not 1D or not contiguous
- `ValueError`: If `memoryview` is read-only (in-place operations require writable memoryview)

**Notes:**
- Modifies the array in-place
- Elements less than `min_val` are set to `min_val`
- Elements greater than `max_val` are set to `max_val`
- Elements within the range are unchanged
- Performance: ~25x faster than Python loops for large arrays

**Example:**
```python
import array
import arrayops as ao

arr = array.array('i', [1, 5, 10, 15, 20])
ao.clip(arr, 5.0, 15.0)
print(list(arr))  # [5, 5, 10, 15, 15]
```

---

### `normalize(arr) -> None`

Normalize array elements to the range [0, 1] in-place using min-max normalization.

**Parameters:**
- `arr` (`array.array`, `numpy.ndarray`, or `memoryview`): Input array with numeric type. Must be one of: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`
  - For `numpy.ndarray`: must be 1-dimensional and contiguous (C_CONTIGUOUS or F_CONTIGUOUS)
  - For `memoryview`: must be writable (in-place operations require writable memoryview)
  - Array must not be empty
  - Array must not have all identical values (min != max)

**Returns:**
- `None`: Array is modified in-place

**Raises:**
- `TypeError`: If input is not an `array.array`, `numpy.ndarray`, or `memoryview`
- `TypeError`: If array uses an unsupported typecode
- `TypeError`: If `numpy.ndarray` is not 1D or not contiguous
- `ValueError`: If array is empty
- `ValueError`: If all array elements are identical (min == max, division by zero)
- `ValueError`: If `memoryview` is read-only (in-place operations require writable memoryview)

**Notes:**
- Modifies the array in-place
- Formula: `(x - min) / (max - min)`
- Normalizes all elements to the range [0, 1]
- Requires that min != max (all elements cannot be identical)
- Performance: ~25x faster than computing normalization in pure Python

**Example:**
```python
import array
import arrayops as ao

arr = array.array('f', [10.0, 20.0, 30.0, 40.0, 50.0])
ao.normalize(arr)
print(list(arr))  # [0.0, 0.25, 0.5, 0.75, 1.0]
```

---

## Array Manipulation

### `reverse(arr) -> None`

Reverse the order of array elements in-place.

**Parameters:**
- `arr` (`array.array`, `numpy.ndarray`, or `memoryview`): Input array with numeric type. Must be one of: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`
  - For `numpy.ndarray`: must be 1-dimensional and contiguous (C_CONTIGUOUS or F_CONTIGUOUS)
  - For `memoryview`: must be writable (in-place operations require writable memoryview)

**Returns:**
- `None`: Array is modified in-place

**Raises:**
- `TypeError`: If input is not an `array.array`, `numpy.ndarray`, or `memoryview`
- `TypeError`: If array uses an unsupported typecode
- `TypeError`: If `numpy.ndarray` is not 1D or not contiguous
- `ValueError`: If `memoryview` is read-only (in-place operations require writable memoryview)

**Notes:**
- Modifies the array in-place
- Empty arrays are handled gracefully (no-op)
- Performance: ~30x faster than Python's `arr.reverse()` for large arrays

**Example:**
```python
import array
import arrayops as ao

arr = array.array('i', [1, 2, 3, 4, 5])
ao.reverse(arr)
print(list(arr))  # [5, 4, 3, 2, 1]
```

---

### `sort(arr) -> None`

Sort array elements in ascending order in-place.

**Parameters:**
- `arr` (`array.array`, `numpy.ndarray`, or `memoryview`): Input array with numeric type. Must be one of: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`
  - For `numpy.ndarray`: must be 1-dimensional and contiguous (C_CONTIGUOUS or F_CONTIGUOUS)
  - For `memoryview`: must be writable (in-place operations require writable memoryview)

**Returns:**
- `None`: Array is modified in-place

**Raises:**
- `TypeError`: If input is not an `array.array`, `numpy.ndarray`, or `memoryview`
- `TypeError`: If array uses an unsupported typecode
- `TypeError`: If `numpy.ndarray` is not 1D or not contiguous
- `ValueError`: If `memoryview` is read-only (in-place operations require writable memoryview)

**Notes:**
- Modifies the array in-place
- Uses stable sort (ascending order)
- Empty arrays are handled gracefully (no-op)
- Performance: ~10x faster than Python's `arr.sort()` for large arrays

**Example:**
```python
import array
import arrayops as ao

arr = array.array('i', [5, 2, 8, 1, 9])
ao.sort(arr)
print(list(arr))  # [1, 2, 5, 8, 9]
```

---

### `unique(arr) -> array.array | numpy.ndarray`

Return a new array containing unique elements from the input array, sorted in ascending order.

**Parameters:**
- `arr` (`array.array`, `numpy.ndarray`, or `memoryview`): Input array with numeric type. Must be one of: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`
  - For `numpy.ndarray`: must be 1-dimensional and contiguous (C_CONTIGUOUS or F_CONTIGUOUS)
  - For `memoryview`: read-only or writable memoryview objects are supported

**Returns:**
- `array.array` or `numpy.ndarray`: New array containing unique elements in sorted order
  - Returns `numpy.ndarray` if input is `numpy.ndarray`
  - Returns `array.array` if input is `array.array` or `memoryview`
  - Result type matches input type

**Raises:**
- `TypeError`: If input is not an `array.array`, `numpy.ndarray`, or `memoryview`
- `TypeError`: If array uses an unsupported typecode
- `TypeError`: If `numpy.ndarray` is not 1D or not contiguous

**Notes:**
- Returns a new array; the original array is not modified
- Result contains unique elements in sorted (ascending) order
- Empty arrays return an empty array of the same type
- Performance: ~20x faster than Python's `list(set(arr))` for large arrays

**Example:**
```python
import array
import arrayops as ao

arr = array.array('i', [5, 2, 8, 2, 1, 5, 9])
unique_arr = ao.unique(arr)
print(list(unique_arr))  # [1, 2, 5, 8, 9]
```

---

## Error Handling

All functions provide clear, descriptive error messages:

```python
import array
import arrayops as ao

# Wrong type
try:
    ao.sum([1, 2, 3])  # TypeError: Expected array.array
except TypeError as e:
    print(e)

# Unsupported typecode
try:
    arr = array.array('c', b'abc')  # Character array
    ao.sum(arr)  # TypeError: Unsupported typecode: 'c'
except TypeError as e:
    print(e)

# Non-callable function
try:
    arr = array.array('i', [1, 2, 3])
    ao.map(arr, "not a function")  # TypeError: Expected callable
except TypeError as e:
    print(e)

# Empty array with reduce (no initial)
try:
    empty = array.array('i', [])
    ao.reduce(empty, lambda acc, x: acc + x)  # ValueError: reduce() of empty array
except ValueError as e:
    print(e)
```

## Type Safety

All functions validate their inputs:
- Type checking ensures only `array.array`, `numpy.ndarray` (1D, contiguous), `memoryview`, or Apache Arrow buffers/arrays are accepted
- Typecode validation ensures only numeric types are supported
- Clear error messages guide users to correct usage

For static type checking with mypy, type stubs are provided in `ao._arrayops`.

## NumPy Integration

`arrayops` supports `numpy.ndarray` objects with the following requirements:
- Arrays must be 1-dimensional (`ndim == 1`)
- Arrays must be contiguous (either `C_CONTIGUOUS` or `F_CONTIGUOUS`)
- All numeric dtypes are supported (int8/16/32/64, uint8/16/32/64, float32/64)
- NumPy is an optional dependency - the package works without NumPy installed

When NumPy arrays are used with `map` or `filter`, the result is also a `numpy.ndarray` with the same dtype as the input.

## Memoryview Support

`arrayops` supports Python's built-in `memoryview` objects:
- Works with both read-only and writable memoryviews
- In-place operations (`scale`, `map_inplace`) require writable memoryviews
- All numeric format types are supported
- `map` and `filter` operations return `array.array` (memoryviews are read-only views)

This enables interoperability with binary data, network protocols, and other buffer-like objects.

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

---

### `slice(arr, start=None, end=None) -> memoryview`

Create a zero-copy slice view of an array.

**Parameters:**
- `arr` (`array.array`, `numpy.ndarray`, or `memoryview`): Input array with numeric type. Must be one of: `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`, `f`, `d`
  - For `numpy.ndarray`: must be 1-dimensional and contiguous
  - For `memoryview`: read-only or writable memoryviews are supported
- `start` (`int`, optional): Start index (default: 0)
- `end` (`int`, optional): End index (default: length of array)

**Returns:**
- `memoryview`: A zero-copy view of the specified slice. The view shares memory with the original array, so modifications to the original array will be reflected in the view.

**Raises:**
- `TypeError`: If input is not an `array.array`, `numpy.ndarray`, or `memoryview`
- `TypeError`: If array uses an unsupported typecode
- `ValueError`: If slice indices are invalid (start > end, start > length, end > length)

**Notes:**
- Returns a `memoryview` object, not a new array
- Zero-copy operation - no data is duplicated
- Modifying the original array will affect the view
- Empty slices return an empty memoryview

**Example:**
```python
import array
import arrayops as ao

# Basic slicing
arr = array.array('i', [1, 2, 3, 4, 5])
view = ao.slice(arr, 1, 4)
print(list(view))  # [2, 3, 4]
print(type(view))  # <class 'memoryview'>

# Slice with defaults
view1 = ao.slice(arr, None, 3)  # [1, 2, 3]
view2 = ao.slice(arr, 2, None)  # [3, 4, 5]
view3 = ao.slice(arr, None, None)  # [1, 2, 3, 4, 5]

# Zero-copy: modifications to original affect view
arr[2] = 99
print(list(view))  # [2, 99, 4]  (view reflects the change)
```

**See also:**
- [Examples: Zero-Copy Slicing](examples.md#zero-copy-slicing)

---

### `array_iterator(arr) -> ArrayIterator`

Create an efficient Rust-optimized iterator for an array-like object.

**Parameters:**
- `arr` (`array.array`, `numpy.ndarray`, `memoryview`, or Arrow buffer/array): Input array with numeric type

**Returns:**
- `ArrayIterator`: An iterator object that supports Python's iterator protocol

**Raises:**
- `TypeError`: If input is not an `array.array`, `numpy.ndarray`, `memoryview`, or Arrow buffer/array
- `TypeError`: If array uses an unsupported typecode

**Notes:**
- The iterator uses Rust's buffer protocol for efficient element access
- Works with all supported array types (array.array, numpy.ndarray, memoryview)
- Compatible with Python's iterator protocol (for loops, list(), sum(), etc.)
- Zero-copy or near-zero-copy iteration from Rust buffers

**Example:**
```python
import array
import arrayops as ao

arr = array.array('i', [1, 2, 3, 4, 5])

# Create iterator
it = ao.array_iterator(arr)

# Use in for loop
for value in it:
    print(value)  # Prints: 1, 2, 3, 4, 5

# Convert to list
it = ao.array_iterator(arr)
values = list(it)
print(values)  # [1, 2, 3, 4, 5]

# Use with built-in functions
it = ao.array_iterator(arr)
total = sum(it)
print(total)  # 15
```

**See also:**
- [Examples: Iterator Protocol](examples.md#iterator-protocol)
- [ArrayIterator Class](#arrayiterator-class)

---

### `ArrayIterator` Class

An efficient Rust-optimized iterator for array-like objects.

The `ArrayIterator` class provides efficient iteration over array types using Rust's buffer protocol access. It implements Python's iterator protocol and can be used in for loops, list comprehensions, and other iterator contexts.

#### Constructor

**`ArrayIterator(source)`**

Create a new ArrayIterator from a source array.

- `source` (`array.array`, `numpy.ndarray`, `memoryview`, or Arrow buffer/array): The source array to iterate over
- Returns: New `ArrayIterator` instance

#### Methods

**`__iter__() -> ArrayIterator`**

Return the iterator itself (required by iterator protocol).

- Returns: The iterator object itself

**`__next__() -> Union[int, float]`**

Return the next element in the iteration.

- Returns: The next element in the array (int for integer arrays, float for float arrays)
- Raises: `StopIteration` when there are no more elements

**Example:**
```python
import array
import arrayops as ao

arr = array.array('i', [1, 2, 3, 4, 5])

# Create iterator
it = ao.array_iterator(arr)

# Use iterator protocol methods
it2 = iter(it)  # __iter__() called
first = next(it2)  # 1 (__next__() called)
second = next(it2)  # 2
third = next(it2)  # 3

# Use in for loop
for value in ao.array_iterator(arr):
    print(value)  # Prints: 1, 2, 3, 4, 5
```

**See also:**
- [Examples: Iterator Protocol](examples.md#iterator-protocol)
- [`array_iterator()` function](#array_iteratorarr---arrayiterator)

---

### `lazy_array(arr) -> LazyArray`

Create a lazy array that can chain operations without intermediate allocations.

**Parameters:**
- `arr` (`array.array`, `numpy.ndarray`, or `memoryview`): Input array with numeric type

**Returns:**
- `LazyArray`: A lazy array object that supports chaining operations

**Raises:**
- `TypeError`: If input is not an `array.array`, `numpy.ndarray`, or `memoryview`

**Notes:**
- Lazy arrays defer execution until `collect()` is called
- Operations can be chained: `lazy.map(fn).filter(pred).collect()`
- More memory-efficient than multiple separate `map()` and `filter()` calls
- Result is cached after first `collect()` call

**Example:**
```python
import array
import arrayops as ao

arr = array.array('i', [1, 2, 3, 4, 5])

# Create lazy array and chain operations
lazy = ao.lazy_array(arr)
result = lazy.map(lambda x: x * 2).filter(lambda x: x > 5).collect()
print(list(result))  # [6, 8, 10]

# Multiple map operations
lazy = ao.lazy_array(arr)
result = lazy.map(lambda x: x * 2).map(lambda x: x + 1).collect()
print(list(result))  # [3, 5, 7, 9, 11]
```

**See also:**
- [Examples: Lazy Evaluation](examples.md#lazy-evaluation)

---

### `LazyArray` Class

A lazy array wrapper that chains operations without intermediate allocations.

#### Methods

**`map(function) -> LazyArray`**

Apply a function to each element (returns a new LazyArray, does not execute).

- `function` (`callable`): Function to apply to each element
- Returns: New `LazyArray` with the map operation added to the chain

**`filter(predicate) -> LazyArray`**

Filter elements based on a predicate (returns a new LazyArray, does not execute).

- `predicate` (`callable`): Predicate function that returns True/False
- Returns: New `LazyArray` with the filter operation added to the chain

**`collect() -> array.array | numpy.ndarray`**

Execute all chained operations and return the result.

- Returns: Result array (same type as input: `array.array` or `numpy.ndarray`)

**`source() -> array.array | numpy.ndarray`**

Get the source array.

- Returns: Original source array

**`len() -> int`**

Get the number of operations in the chain.

- Returns: Number of operations in the chain

**`__iter__() -> ArrayIterator`**

Iterator protocol: evaluate lazy chain and return an iterator.

- Returns: An `ArrayIterator` over the evaluated array result
- Notes: This method evaluates the lazy chain (if not already cached) and returns an iterator over the result, enabling direct iteration over LazyArray objects

**Example:**
```python
import array
import arrayops as ao

arr = array.array('i', [1, 2, 3, 4, 5])
lazy = ao.lazy_array(arr)

# Chain multiple operations
lazy = lazy.map(lambda x: x * 2)
lazy = lazy.filter(lambda x: x > 5)

# Check chain length
print(lazy.len())  # 2

# Execute and get result
result = lazy.collect()
print(list(result))  # [6, 8, 10]

# Get source
source = lazy.source()
print(list(source))  # [1, 2, 3, 4, 5]

# Iterate directly (evaluates chain automatically)
lazy = ao.lazy_array(arr)
lazy = lazy.map(lambda x: x * 2).filter(lambda x: x > 5)
for value in lazy:
    print(value)  # Prints: 6, 8, 10
```

---

## Arrow Buffer Support

`arrayops` supports Apache Arrow buffers and arrays (`pyarrow.Buffer`, `pyarrow.Array`, `pyarrow.ChunkedArray`):
- All operations work transparently with Arrow arrays
- Arrow arrays are detected automatically
- Results are returned as Arrow arrays when Arrow input is used
- Requires `pyarrow` to be installed (optional dependency)

## Related Documentation

- [Examples and Cookbook](examples.md) - Practical usage examples
- [Performance Guide](performance.md) - Performance analysis and optimization
- [Design Document](../design.md) - Architecture and implementation details
- [Troubleshooting](troubleshooting.md) - Common issues and solutions

