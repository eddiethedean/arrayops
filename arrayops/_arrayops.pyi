"""Type stubs for arrayops._arrayops Rust extension module."""

import array
from typing import Any, Callable, Optional, TYPE_CHECKING, Union

if TYPE_CHECKING:
    try:
        import numpy as np
    except ImportError:
        # NumPy may not be installed - define a stub type
        from typing import Protocol

        class np:  # type: ignore[no-redef]
            class ndarray(Protocol):  # type: ignore[misc]
                ...

else:
    # At runtime, try to import numpy
    try:
        import numpy as np
    except ImportError:
        # Define a minimal stub for runtime if numpy not available
        class np:  # type: ignore[no-redef]
            class ndarray:  # type: ignore[misc]
                ...

# Supported input types: array.array, numpy.ndarray, memoryview, or Apache Arrow buffers/arrays
_ArrayLike = Union[array.array, "np.ndarray", memoryview]

def sum(arr: _ArrayLike) -> Union[int, float]:
    """
    Compute the sum of all elements in an array.

    This function provides a fast, Rust-accelerated sum operation that is
    significantly faster than Python's built-in sum() for large arrays. The
    operation uses zero-copy buffer access for optimal performance.

    Args:
        arr: Input array with numeric type. Must be one of:
            - ``array.array`` with typecode: ``b``, ``B``, ``h``, ``H``, ``i``, ``I``, ``l``, ``L``, ``f``, ``d``
            - ``numpy.ndarray``: must be 1-dimensional and contiguous (C_CONTIGUOUS or F_CONTIGUOUS)
            - ``memoryview``: read-only or writable memoryviews are supported
            - Apache Arrow buffers/arrays (``pyarrow.Buffer``, ``pyarrow.Array``, ``pyarrow.ChunkedArray``)

    Returns:
        Union[int, float]: The sum of all elements.
            - Returns ``int`` for integer arrays (typecodes: ``b``, ``B``, ``h``, ``H``, ``i``, ``I``, ``l``, ``L``)
            - Returns ``float`` for float arrays (typecodes: ``f``, ``d``)

    Raises:
        TypeError: If input is not an ``array.array``, ``numpy.ndarray``, ``memoryview``, or Arrow buffer/array
        TypeError: If array uses an unsupported typecode
        TypeError: If ``numpy.ndarray`` is not 1D or not contiguous

    Notes:
        - Empty arrays return ``0`` (integer) or ``0.0`` (float)
        - Integer overflow follows Python's semantics (promotion to larger types)
        - Performance: ~100x faster than Python's built-in ``sum()`` for large arrays
        - Parallel execution: When built with ``--features parallel``, arrays with 1,000+ elements
          automatically use parallel processing for additional speedup on multi-core systems
        - SIMD optimization: Infrastructure available via ``--features simd`` (full implementation pending)

    Examples:
        >>> import array
        >>> import arrayops as ao
        >>> arr = array.array('i', [1, 2, 3, 4, 5])
        >>> ao.sum(arr)
        15
        >>> farr = array.array('f', [1.5, 2.5, 3.5])
        >>> ao.sum(farr)
        7.5
    """
    ...

def scale(arr: _ArrayLike, factor: float) -> None:
    """
    Scale all elements of an array in-place by a factor.

    This function multiplies each element of the array by the given factor,
    modifying the array in-place. This is significantly faster than Python
    loops for large arrays.

    Args:
        arr: Input array with numeric type (modified in-place). Must be one of:
            - ``array.array`` with typecode: ``b``, ``B``, ``h``, ``H``, ``i``, ``I``, ``l``, ``L``, ``f``, ``d``
            - ``numpy.ndarray``: must be 1-dimensional and contiguous
            - ``memoryview``: must be writable (read-only memoryviews raise ValueError)
            - Apache Arrow buffers/arrays
        factor: Scaling factor to multiply each element by

    Returns:
        None: This function modifies the array in-place and returns nothing

    Raises:
        TypeError: If input is not an ``array.array``, ``numpy.ndarray``, ``memoryview``, or Arrow buffer/array
        TypeError: If array uses an unsupported typecode
        TypeError: If ``numpy.ndarray`` is not 1D or not contiguous
        ValueError: If ``memoryview`` is read-only

    Notes:
        - The array is modified in-place; no new array is created
        - For integer arrays, the factor is cast to the array's integer type
        - Empty arrays are handled gracefully (no error, array remains empty)
        - Performance: ~50x faster than Python loops for large arrays
        - Parallel execution: When built with ``--features parallel``, arrays with 1,000+ elements
          automatically use parallel processing for additional speedup on multi-core systems

    Examples:
        >>> import array
        >>> import arrayops as ao
        >>> arr = array.array('i', [1, 2, 3, 4, 5])
        >>> ao.scale(arr, 2.0)
        >>> list(arr)
        [2, 4, 6, 8, 10]
    """
    ...

def map(
    arr: _ArrayLike, fn: Callable[[Union[int, float]], Union[int, float]]
) -> Union[array.array, "np.ndarray"]:
    """
    Apply a function to each element, returning a new array.

    This function creates a new array by applying the given function to each
    element of the input array. The result preserves the input array type
    (NumPy arrays return NumPy arrays, array.array returns array.array).

    Args:
        arr: Input array with numeric type. Must be one of:
            - ``array.array`` with typecode: ``b``, ``B``, ``h``, ``H``, ``i``, ``I``, ``l``, ``L``, ``f``, ``d``
            - ``numpy.ndarray``: must be 1-dimensional and contiguous
            - ``memoryview``: read-only or writable memoryviews are supported
            - Apache Arrow buffers/arrays
        fn: Callable function that takes a single numeric value and returns a numeric value.
            Can be a lambda, named function, or any callable object.

    Returns:
        Union[array.array, np.ndarray]: New array with mapped values.
            - Returns ``numpy.ndarray`` if input is ``numpy.ndarray`` or Arrow array
            - Returns ``array.array`` if input is ``array.array`` or ``memoryview``

    Raises:
        TypeError: If input is not an ``array.array``, ``numpy.ndarray``, ``memoryview``, or Arrow buffer/array
        TypeError: If ``fn`` is not callable
        TypeError: If array uses an unsupported typecode

    Notes:
        - Creates a new array; the original array is not modified
        - Performance: ~20x faster than Python list comprehension for large arrays
        - The function is called once per element in the array
        - Type preservation: result type matches input type (NumPy → NumPy, array.array → array.array)

    Examples:
        >>> import array
        >>> import arrayops as ao
        >>> arr = array.array('i', [1, 2, 3, 4, 5])
        >>> doubled = ao.map(arr, lambda x: x * 2)
        >>> list(doubled)
        [2, 4, 6, 8, 10]
    """
    ...

def map_inplace(
    arr: _ArrayLike, fn: Callable[[Union[int, float]], Union[int, float]]
) -> None:
    """
    Apply a function to each element in-place.

    This function modifies the array in-place by applying the given function
    to each element. This is more memory-efficient than ``map()`` when you
    don't need to preserve the original array.

    Args:
        arr: Input array with numeric type (modified in-place). Must be one of:
            - ``array.array`` with typecode: ``b``, ``B``, ``h``, ``H``, ``i``, ``I``, ``l``, ``L``, ``f``, ``d``
            - ``numpy.ndarray``: must be 1-dimensional and contiguous
            - ``memoryview``: must be writable (read-only memoryviews raise ValueError)
            - Apache Arrow buffers/arrays
        fn: Callable function that takes a single numeric value and returns a numeric value.
            Can be a lambda, named function, or any callable object.

    Returns:
        None: This function modifies the array in-place and returns nothing

    Raises:
        TypeError: If input is not an ``array.array``, ``numpy.ndarray``, ``memoryview``, or Arrow buffer/array
        TypeError: If ``fn`` is not callable
        TypeError: If array uses an unsupported typecode
        ValueError: If ``memoryview`` is read-only

    Notes:
        - Modifies the array in-place; no new array is created
        - More memory-efficient than ``map()`` when you don't need the original array
        - Performance: ~15x faster than Python loops for large arrays
        - The function is called once per element in the array

    Examples:
        >>> import array
        >>> import arrayops as ao
        >>> arr = array.array('i', [1, 2, 3, 4, 5])
        >>> ao.map_inplace(arr, lambda x: x * 2)
        >>> list(arr)
        [2, 4, 6, 8, 10]
    """
    ...

def filter(
    arr: _ArrayLike, predicate: Callable[[Union[int, float]], bool]
) -> Union[array.array, "np.ndarray"]:
    """
    Return a new array with elements that pass the predicate function.

    This function creates a new array containing only the elements for which
    the predicate function returns ``True``. The result preserves the input
    array type.

    Args:
        arr: Input array with numeric type. Must be one of:
            - ``array.array`` with typecode: ``b``, ``B``, ``h``, ``H``, ``i``, ``I``, ``l``, ``L``, ``f``, ``d``
            - ``numpy.ndarray``: must be 1-dimensional and contiguous
            - ``memoryview``: read-only or writable memoryviews are supported
            - Apache Arrow buffers/arrays
        predicate: Callable function that takes a single numeric value and returns a boolean.
            Should return ``True`` for elements to keep, ``False`` for elements to filter out.
            Can be a lambda, named function, or any callable object.

    Returns:
        Union[array.array, np.ndarray]: New array with filtered elements.
            - Returns ``numpy.ndarray`` if input is ``numpy.ndarray`` or Arrow array
            - Returns ``array.array`` if input is ``array.array`` or ``memoryview``

    Raises:
        TypeError: If input is not an ``array.array``, ``numpy.ndarray``, ``memoryview``, or Arrow buffer/array
        TypeError: If ``predicate`` is not callable
        TypeError: If array uses an unsupported typecode

    Notes:
        - Creates a new array; the original array is not modified
        - Empty result arrays are handled gracefully
        - Performance: ~15x faster than Python list comprehension for large arrays
        - The predicate function is called once per element in the array
        - Type preservation: result type matches input type (NumPy → NumPy, array.array → array.array)

    Examples:
        >>> import array
        >>> import arrayops as ao
        >>> arr = array.array('i', [1, 2, 3, 4, 5, 6])
        >>> evens = ao.filter(arr, lambda x: x % 2 == 0)
        >>> list(evens)
        [2, 4, 6]
    """
    ...

def reduce(
    arr: _ArrayLike,
    fn: Callable[[Any, Union[int, float]], Any],
    initial: Optional[Any] = None,
) -> Any:
    """
    Fold array elements into a single value using a binary function.

    This function applies a binary function cumulatively to the elements of
    the array, reducing it to a single value. This is similar to Python's
    ``functools.reduce()`` but significantly faster for large arrays.

    Args:
        arr: Input array with numeric type. Must be one of:
            - ``array.array`` with typecode: ``b``, ``B``, ``h``, ``H``, ``i``, ``I``, ``l``, ``L``, ``f``, ``d``
            - ``numpy.ndarray``: must be 1-dimensional and contiguous
            - ``memoryview``: read-only or writable memoryviews are supported
            - Apache Arrow buffers/arrays
        fn: Binary callable function that takes two arguments:
            - Accumulator: the accumulated value (initially ``initial`` or first element)
            - Current element: the current array element
            Returns the new accumulated value.
        initial: Optional initial value for the accumulator.
            If ``None`` and array is empty, raises ``ValueError``.
            If ``None`` and array is non-empty, uses the first element as initial value.

    Returns:
        Any: The accumulated/folded value. Type depends on the function and initial value.

    Raises:
        TypeError: If input is not an ``array.array``, ``numpy.ndarray``, ``memoryview``, or Arrow buffer/array
        TypeError: If ``fn`` is not callable
        ValueError: If array is empty and ``initial`` is ``None``

    Notes:
        - Performance: ~25x faster than Python's ``functools.reduce()`` for large arrays
        - The function is called once per element (except the first if no initial value)
        - Left-associative: processes elements from left to right
        - Type inference: return type depends on the function's return type and initial value type

    Examples:
        >>> import array
        >>> import arrayops as ao
        >>> arr = array.array('i', [1, 2, 3, 4, 5])
        >>> ao.reduce(arr, lambda acc, x: acc + x)
        15
        >>> ao.reduce(arr, lambda acc, x: acc * x, initial=1)
        120
    """
    ...

def mean(arr: _ArrayLike) -> float:
    """
    Compute the arithmetic mean (average) of all elements in an array.

    This function calculates the mean by summing all elements and dividing
    by the number of elements. The result is always a float, even for
    integer arrays.

    Args:
        arr: Input array with numeric type. Must be one of:
            - ``array.array`` with typecode: ``b``, ``B``, ``h``, ``H``, ``i``, ``I``, ``l``, ``L``, ``f``, ``d``
            - ``numpy.ndarray``: must be 1-dimensional and contiguous
            - ``memoryview``: read-only or writable memoryviews are supported
            - Apache Arrow buffers/arrays

    Returns:
        float: The arithmetic mean of all elements. Always returns a float,
            even for integer arrays.

    Raises:
        TypeError: If input is not an ``array.array``, ``numpy.ndarray``, ``memoryview``, or Arrow buffer/array
        ValueError: If array is empty

    Notes:
        - Always returns a float, even for integer arrays
        - Performance: ~50x faster than computing mean in pure Python for large arrays
        - Parallel execution: When built with ``--features parallel``, arrays with 1,000+ elements
          automatically use parallel processing for additional speedup on multi-core systems
        - Formula: sum(elements) / len(array)

    Examples:
        >>> import array
        >>> import arrayops as ao
        >>> arr = array.array('i', [1, 2, 3, 4, 5])
        >>> ao.mean(arr)
        3.0
        >>> farr = array.array('f', [10.0, 20.0, 30.0])
        >>> ao.mean(farr)
        20.0
    """
    ...

def min(arr: _ArrayLike) -> Union[int, float]:  # noqa: A001
    """
    Find the minimum value in an array.

    This function returns the smallest element in the array. For integer
    arrays, returns an integer; for float arrays, returns a float.

    Args:
        arr: Input array with numeric type. Must be one of:
            - ``array.array`` with typecode: ``b``, ``B``, ``h``, ``H``, ``i``, ``I``, ``l``, ``L``, ``f``, ``d``
            - ``numpy.ndarray``: must be 1-dimensional and contiguous
            - ``memoryview``: read-only or writable memoryviews are supported
            - Apache Arrow buffers/arrays

    Returns:
        Union[int, float]: The minimum value in the array.
            - Returns ``int`` for integer arrays
            - Returns ``float`` for float arrays

    Raises:
        TypeError: If input is not an ``array.array``, ``numpy.ndarray``, ``memoryview``, or Arrow buffer/array
        ValueError: If array is empty

    Notes:
        - Returns type matching array element type (int for integer arrays, float for float arrays)
        - Performance: ~30x faster than Python's built-in ``min()`` for large arrays
        - Parallel execution: When built with ``--features parallel``, arrays with 50,000+ elements
          automatically use parallel processing for additional speedup on multi-core systems

    Examples:
        >>> import array
        >>> import arrayops as ao
        >>> arr = array.array('i', [5, 2, 8, 1, 9])
        >>> ao.min(arr)
        1
        >>> farr = array.array('f', [3.5, 1.2, 7.8, 0.5])
        >>> ao.min(farr)
        0.5
    """
    ...

def max(arr: _ArrayLike) -> Union[int, float]:  # noqa: A001
    """
    Find the maximum value in an array.

    This function returns the largest element in the array. For integer
    arrays, returns an integer; for float arrays, returns a float.

    Args:
        arr: Input array with numeric type. Must be one of:
            - ``array.array`` with typecode: ``b``, ``B``, ``h``, ``H``, ``i``, ``I``, ``l``, ``L``, ``f``, ``d``
            - ``numpy.ndarray``: must be 1-dimensional and contiguous
            - ``memoryview``: read-only or writable memoryviews are supported
            - Apache Arrow buffers/arrays

    Returns:
        Union[int, float]: The maximum value in the array.
            - Returns ``int`` for integer arrays
            - Returns ``float`` for float arrays

    Raises:
        TypeError: If input is not an ``array.array``, ``numpy.ndarray``, ``memoryview``, or Arrow buffer/array
        ValueError: If array is empty

    Notes:
        - Returns type matching array element type (int for integer arrays, float for float arrays)
        - Performance: ~30x faster than Python's built-in ``max()`` for large arrays
        - Parallel execution: When built with ``--features parallel``, arrays with 50,000+ elements
          automatically use parallel processing for additional speedup on multi-core systems

    Examples:
        >>> import array
        >>> import arrayops as ao
        >>> arr = array.array('i', [5, 2, 8, 1, 9])
        >>> ao.max(arr)
        9
        >>> farr = array.array('f', [3.5, 1.2, 7.8, 0.5])
        >>> ao.max(farr)
        7.8
    """
    ...

def std(arr: _ArrayLike) -> float:
    """
    Compute the population standard deviation of all elements in an array.

    This function calculates the population standard deviation using the formula:
    sqrt(sum((x - mean)^2) / n). The result is always a float.

    Args:
        arr: Input array with numeric type. Must be one of:
            - ``array.array`` with typecode: ``b``, ``B``, ``h``, ``H``, ``i``, ``I``, ``l``, ``L``, ``f``, ``d``
            - ``numpy.ndarray``: must be 1-dimensional and contiguous
            - ``memoryview``: read-only or writable memoryviews are supported
            - Apache Arrow buffers/arrays

    Returns:
        float: The population standard deviation. Always returns a float.

    Raises:
        TypeError: If input is not an ``array.array``, ``numpy.ndarray``, ``memoryview``, or Arrow buffer/array
        ValueError: If array is empty

    Notes:
        - Always returns a float
        - Uses population standard deviation formula: sqrt(sum((x - mean)^2) / n)
        - Performance: ~40x faster than computing std in pure Python for large arrays
        - For sample standard deviation, multiply result by sqrt(n/(n-1))

    Examples:
        >>> import array
        >>> import arrayops as ao
        >>> arr = array.array('i', [1, 2, 3, 4, 5])
        >>> ao.std(arr)
        1.4142135623730951
    """
    ...

def var(arr: _ArrayLike) -> float:
    """
    Compute the population variance of all elements in an array.

    This function calculates the population variance using the formula:
    sum((x - mean)^2) / n. The result is always a float.

    Args:
        arr: Input array with numeric type. Must be one of:
            - ``array.array`` with typecode: ``b``, ``B``, ``h``, ``H``, ``i``, ``I``, ``l``, ``L``, ``f``, ``d``
            - ``numpy.ndarray``: must be 1-dimensional and contiguous
            - ``memoryview``: read-only or writable memoryviews are supported
            - Apache Arrow buffers/arrays

    Returns:
        float: The population variance. Always returns a float.

    Raises:
        TypeError: If input is not an ``array.array``, ``numpy.ndarray``, ``memoryview``, or Arrow buffer/array
        ValueError: If array is empty

    Notes:
        - Always returns a float
        - Uses population variance formula: sum((x - mean)^2) / n
        - Performance: ~40x faster than computing variance in pure Python for large arrays
        - Standard deviation is the square root of variance: std = sqrt(var)
        - For sample variance, multiply result by n/(n-1)

    Examples:
        >>> import array
        >>> import arrayops as ao
        >>> arr = array.array('i', [1, 2, 3, 4, 5])
        >>> ao.var(arr)
        2.0
    """
    ...

def median(arr: _ArrayLike) -> Union[int, float]:
    """
    Find the median value in an array.

    This function returns the median (middle value) of the array. For arrays
    with an even number of elements, returns the lower median. The array is
    sorted in-place to find the median.

    Args:
        arr: Input array with numeric type. Must be one of:
            - ``array.array`` with typecode: ``b``, ``B``, ``h``, ``H``, ``i``, ``I``, ``l``, ``L``, ``f``, ``d``
            - ``numpy.ndarray``: must be 1-dimensional and contiguous
            - ``memoryview``: read-only or writable memoryviews are supported
            - Apache Arrow buffers/arrays

    Returns:
        Union[int, float]: The median value.
            - Returns ``int`` for integer arrays
            - Returns ``float`` for float arrays
            - For even-length arrays, returns the lower median

    Raises:
        TypeError: If input is not an ``array.array``, ``numpy.ndarray``, ``memoryview``, or Arrow buffer/array
        ValueError: If array is empty

    Notes:
        - Returns type matching array element type (int for integer arrays, float for float arrays)
        - For even-length arrays, returns the lower median (element at index (n-1)/2 after sorting)
        - Performance: ~20x faster than computing median in pure Python for large arrays
        - The array may be sorted internally, but original order is preserved when possible

    Examples:
        >>> import array
        >>> import arrayops as ao
        >>> arr = array.array('i', [5, 2, 8, 1, 9])
        >>> ao.median(arr)
        5
        >>> arr2 = array.array('i', [1, 2, 3, 4])
        >>> ao.median(arr2)  # Even length: returns lower median
        2
    """
    ...

def add(arr1: _ArrayLike, arr2: _ArrayLike) -> Union[array.array, "np.ndarray"]:
    """
    Perform element-wise addition of two arrays.

    This function adds corresponding elements from two arrays element-wise,
    returning a new array with the results. Both arrays must have the same
    length.

    Args:
        arr1: First input array with numeric type. Must be one of:
            - ``array.array`` with typecode: ``b``, ``B``, ``h``, ``H``, ``i``, ``I``, ``l``, ``L``, ``f``, ``d``
            - ``numpy.ndarray``: must be 1-dimensional and contiguous
            - ``memoryview``: read-only or writable memoryviews are supported
            - Apache Arrow buffers/arrays
        arr2: Second input array with numeric type. Must be the same type and length as ``arr1``.

    Returns:
        Union[array.array, np.ndarray]: New array with element-wise sum.
            - Returns ``numpy.ndarray`` if input is ``numpy.ndarray`` or Arrow array
            - Returns ``array.array`` if input is ``array.array`` or ``memoryview``
            - Result type matches input type

    Raises:
        TypeError: If input is not an ``array.array``, ``numpy.ndarray``, ``memoryview``, or Arrow buffer/array
        ValueError: If arrays have different lengths

    Notes:
        - Creates a new array; input arrays are not modified
        - Both arrays must have the same length
        - Performance: ~30x faster than Python loops for large arrays
        - Parallel execution: When built with ``--features parallel``, arrays with 1,000+ elements
          automatically use parallel processing for additional speedup on multi-core systems
        - Type preservation: result type matches input type

    Examples:
        >>> import array
        >>> import arrayops as ao
        >>> arr1 = array.array('i', [1, 2, 3, 4, 5])
        >>> arr2 = array.array('i', [10, 20, 30, 40, 50])
        >>> result = ao.add(arr1, arr2)
        >>> list(result)
        [11, 22, 33, 44, 55]
    """
    ...

def multiply(arr1: _ArrayLike, arr2: _ArrayLike) -> Union[array.array, "np.ndarray"]:
    """
    Perform element-wise multiplication of two arrays.

    This function multiplies corresponding elements from two arrays element-wise,
    returning a new array with the results. Both arrays must have the same
    length.

    Args:
        arr1: First input array with numeric type. Must be one of:
            - ``array.array`` with typecode: ``b``, ``B``, ``h``, ``H``, ``i``, ``I``, ``l``, ``L``, ``f``, ``d``
            - ``numpy.ndarray``: must be 1-dimensional and contiguous
            - ``memoryview``: read-only or writable memoryviews are supported
            - Apache Arrow buffers/arrays
        arr2: Second input array with numeric type. Must be the same type and length as ``arr1``.

    Returns:
        Union[array.array, np.ndarray]: New array with element-wise product.
            - Returns ``numpy.ndarray`` if input is ``numpy.ndarray`` or Arrow array
            - Returns ``array.array`` if input is ``array.array`` or ``memoryview``
            - Result type matches input type

    Raises:
        TypeError: If input is not an ``array.array``, ``numpy.ndarray``, ``memoryview``, or Arrow buffer/array
        ValueError: If arrays have different lengths

    Notes:
        - Creates a new array; input arrays are not modified
        - Both arrays must have the same length
        - Performance: ~30x faster than Python loops for large arrays
        - Parallel execution: When built with ``--features parallel``, arrays with 1,000+ elements
          automatically use parallel processing for additional speedup on multi-core systems
        - Type preservation: result type matches input type

    Examples:
        >>> import array
        >>> import arrayops as ao
        >>> arr1 = array.array('i', [1, 2, 3, 4, 5])
        >>> arr2 = array.array('i', [10, 20, 30, 40, 50])
        >>> result = ao.multiply(arr1, arr2)
        >>> list(result)
        [10, 40, 90, 160, 250]
    """
    ...

def clip(
    arr: _ArrayLike, min_val: Union[int, float], max_val: Union[int, float]
) -> None:
    """
    Clip array elements to a specified range in-place.

    This function modifies the array so that all elements are within the range
    [min_val, max_val]. Elements less than min_val are set to min_val, and
    elements greater than max_val are set to max_val.

    Args:
        arr: Input array with numeric type (modified in-place). Must be one of:
            - ``array.array`` with typecode: ``b``, ``B``, ``h``, ``H``, ``i``, ``I``, ``l``, ``L``, ``f``, ``d``
            - ``numpy.ndarray``: must be 1-dimensional and contiguous
            - ``memoryview``: must be writable (read-only memoryviews raise ValueError)
            - Apache Arrow buffers/arrays
        min_val: Minimum value. Elements less than this are set to min_val.
        max_val: Maximum value. Elements greater than this are set to max_val.

    Returns:
        None: This function modifies the array in-place and returns nothing

    Raises:
        TypeError: If input is not an ``array.array``, ``numpy.ndarray``, ``memoryview``, or Arrow buffer/array
        ValueError: If min_val > max_val
        ValueError: If ``memoryview`` is read-only

    Notes:
        - Modifies the array in-place; no new array is created
        - Performance: ~25x faster than Python loops for large arrays
        - Parallel execution: When built with ``--features parallel``, arrays with 1,000+ elements
          automatically use parallel processing for additional speedup on multi-core systems
        - Elements are clipped to [min_val, max_val] (inclusive range)

    Examples:
        >>> import array
        >>> import arrayops as ao
        >>> arr = array.array('i', [1, 5, 10, 15, 20])
        >>> ao.clip(arr, 5, 15)
        >>> list(arr)
        [5, 5, 10, 15, 15]
    """
    ...

def normalize(arr: _ArrayLike) -> None:
    """
    Normalize array elements to the range [0, 1] in-place using min-max normalization.

    This function normalizes the array using the formula: (x - min) / (max - min).
    This is useful for machine learning preprocessing and feature scaling.

    Args:
        arr: Input array with numeric type (modified in-place). Must be one of:
            - ``array.array`` with typecode: ``b``, ``B``, ``h``, ``H``, ``i``, ``I``, ``l``, ``L``, ``f``, ``d``
            - ``numpy.ndarray``: must be 1-dimensional and contiguous
            - ``memoryview``: must be writable (read-only memoryviews raise ValueError)
            - Apache Arrow buffers/arrays

    Returns:
        None: This function modifies the array in-place and returns nothing

    Raises:
        TypeError: If input is not an ``array.array``, ``numpy.ndarray``, ``memoryview``, or Arrow buffer/array
        ValueError: If array is empty
        ValueError: If min == max (all elements are the same, cannot normalize)
        ValueError: If ``memoryview`` is read-only

    Notes:
        - Modifies the array in-place; no new array is created
        - Uses min-max normalization: (x - min) / (max - min)
        - Result is always in the range [0.0, 1.0]
        - Performance: ~25x faster than computing normalization in pure Python for large arrays
        - Parallel execution: When built with ``--features parallel``, arrays with 2,000+ elements
          automatically use parallel processing for additional speedup on multi-core systems
        - Best used with float arrays; integer arrays will be converted to float values

    Examples:
        >>> import array
        >>> import arrayops as ao
        >>> arr = array.array('f', [10.0, 20.0, 30.0, 40.0, 50.0])
        >>> ao.normalize(arr)
        >>> list(arr)
        [0.0, 0.25, 0.5, 0.75, 1.0]
    """
    ...

def reverse(arr: _ArrayLike) -> None:
    """
    Reverse the order of array elements in-place.

    This function reverses the order of elements in the array, modifying it
    in-place. The first element becomes the last, and the last becomes the first.

    Args:
        arr: Input array with numeric type (modified in-place). Must be one of:
            - ``array.array`` with typecode: ``b``, ``B``, ``h``, ``H``, ``i``, ``I``, ``l``, ``L``, ``f``, ``d``
            - ``numpy.ndarray``: must be 1-dimensional and contiguous
            - ``memoryview``: must be writable (read-only memoryviews raise ValueError)
            - Apache Arrow buffers/arrays

    Returns:
        None: This function modifies the array in-place and returns nothing

    Raises:
        TypeError: If input is not an ``array.array``, ``numpy.ndarray``, ``memoryview``, or Arrow buffer/array
        ValueError: If ``memoryview`` is read-only

    Notes:
        - Modifies the array in-place; no new array is created
        - Performance: ~30x faster than Python's ``array.reverse()`` for large arrays
        - Empty arrays are handled gracefully (no error, array remains empty)

    Examples:
        >>> import array
        >>> import arrayops as ao
        >>> arr = array.array('i', [1, 2, 3, 4, 5])
        >>> ao.reverse(arr)
        >>> list(arr)
        [5, 4, 3, 2, 1]
    """
    ...

def sort(arr: _ArrayLike) -> None:  # noqa: A001
    """
    Sort array elements in-place in ascending order.

    This function sorts the array elements in ascending order using a stable
    sort algorithm. The array is modified in-place.

    Args:
        arr: Input array with numeric type (modified in-place). Must be one of:
            - ``array.array`` with typecode: ``b``, ``B``, ``h``, ``H``, ``i``, ``I``, ``l``, ``L``, ``f``, ``d``
            - ``numpy.ndarray``: must be 1-dimensional and contiguous
            - ``memoryview``: must be writable (read-only memoryviews raise ValueError)
            - Apache Arrow buffers/arrays

    Returns:
        None: This function modifies the array in-place and returns nothing

    Raises:
        TypeError: If input is not an ``array.array``, ``numpy.ndarray``, ``memoryview``, or Arrow buffer/array
        ValueError: If ``memoryview`` is read-only

    Notes:
        - Modifies the array in-place; no new array is created
        - Uses a stable sort algorithm (preserves relative order of equal elements)
        - Performance: ~10x faster than Python's ``array.sort()`` for large arrays
        - Parallel execution: When built with ``--features parallel``, large arrays
          automatically use parallel processing for additional speedup on multi-core systems
        - Empty arrays are handled gracefully (no error, array remains empty)

    Examples:
        >>> import array
        >>> import arrayops as ao
        >>> arr = array.array('i', [5, 2, 8, 1, 9, 3])
        >>> ao.sort(arr)
        >>> list(arr)
        [1, 2, 3, 5, 8, 9]
    """
    ...

def unique(arr: _ArrayLike) -> _ArrayLike:
    """
    Return unique elements from an array, sorted in ascending order.

    This function returns a new array containing only the unique elements from
    the input array, sorted in ascending order. Duplicate elements are removed.

    Args:
        arr: Input array with numeric type. Must be one of:
            - ``array.array`` with typecode: ``b``, ``B``, ``h``, ``H``, ``i``, ``I``, ``l``, ``L``, ``f``, ``d``
            - ``numpy.ndarray``: must be 1-dimensional and contiguous
            - ``memoryview``: read-only or writable memoryviews are supported
            - Apache Arrow buffers/arrays

    Returns:
        _ArrayLike: New array with unique elements, sorted in ascending order.
            - Returns ``numpy.ndarray`` if input is ``numpy.ndarray`` or Arrow array
            - Returns ``array.array`` if input is ``array.array`` or ``memoryview``
            - Result type matches input type

    Raises:
        TypeError: If input is not an ``array.array``, ``numpy.ndarray``, ``memoryview``, or Arrow buffer/array

    Notes:
        - Creates a new array; the original array is not modified
        - Result contains unique elements in sorted (ascending) order
        - Empty arrays return an empty array of the same type
        - Performance: ~20x faster than Python's ``list(set(arr))`` for large arrays
        - Type preservation: result type matches input type

    Examples:
        >>> import array
        >>> import arrayops as ao
        >>> arr = array.array('i', [5, 2, 8, 2, 1, 5, 9])
        >>> unique_arr = ao.unique(arr)
        >>> list(unique_arr)
        [1, 2, 5, 8, 9]
    """
    ...

def slice(
    arr: _ArrayLike, start: Optional[int] = None, end: Optional[int] = None
) -> memoryview:
    """
    Create a zero-copy slice view of an array.

    This function returns a memoryview that provides a zero-copy view of a
    portion of the array. The view shares memory with the original array,
    so modifications to the original array will be reflected in the view.

    Args:
        arr: Input array with numeric type. Must be one of:
            - ``array.array`` with typecode: ``b``, ``B``, ``h``, ``H``, ``i``, ``I``, ``l``, ``L``, ``f``, ``d``
            - ``numpy.ndarray``: must be 1-dimensional and contiguous
            - ``memoryview``: read-only or writable memoryviews are supported
            - Apache Arrow buffers/arrays
        start: Optional start index (default: 0). If None, starts from the beginning.
        end: Optional end index (default: length of array). If None, goes to the end.

    Returns:
        memoryview: A zero-copy view of the specified slice. The view shares memory
            with the original array, so modifications to the original array will be
            reflected in the view.

    Raises:
        TypeError: If input is not an ``array.array``, ``numpy.ndarray``, ``memoryview``, or Arrow buffer/array
        ValueError: If slice indices are invalid (start > end, start > length, end > length)

    Notes:
        - Returns a ``memoryview`` object, not a new array
        - Zero-copy operation - no data is duplicated
        - Modifying the original array will affect the view
        - Empty slices return an empty memoryview
        - Useful for processing chunks of large arrays without copying data

    Examples:
        >>> import array
        >>> import arrayops as ao
        >>> arr = array.array('i', [1, 2, 3, 4, 5])
        >>> view = ao.slice(arr, 1, 4)
        >>> list(view)
        [2, 3, 4]
        >>> arr[2] = 99  # Modify original
        >>> list(view)  # View reflects the change
        [2, 99, 4]
    """
    ...

def array_iterator(arr: _ArrayLike) -> "ArrayIterator":
    """
    Create an efficient Rust-optimized iterator for an array-like object.

    This function creates an ArrayIterator object that provides efficient
    iteration over array types using Rust's buffer protocol access. The iterator
    is compatible with Python's iterator protocol and can be used in for loops,
    list comprehensions, and other iterator contexts.

    Args:
        arr: Input array with numeric type. Must be one of:
            - ``array.array`` with typecode: ``b``, ``B``, ``h``, ``H``, ``i``, ``I``, ``l``, ``L``, ``f``, ``d``
            - ``numpy.ndarray``: must be 1-dimensional and contiguous
            - ``memoryview``: read-only or writable memoryviews are supported
            - Apache Arrow buffers/arrays

    Returns:
        ArrayIterator: An iterator object that supports Python's iterator protocol.

    Raises:
        TypeError: If input is not an ``array.array``, ``numpy.ndarray``, ``memoryview``, or Arrow buffer/array

    Examples:
        >>> import array
        >>> import arrayops as ao
        >>> arr = array.array('i', [1, 2, 3, 4, 5])
        >>> it = ao.array_iterator(arr)
        >>> list(it)
        [1, 2, 3, 4, 5]
        >>> for x in ao.array_iterator(arr):
        ...     print(x)
        1
        2
        3
        4
        5
    """
    ...

class ArrayIterator:
    """
    An efficient Rust-optimized iterator for array-like objects.

    ArrayIterator provides efficient iteration over array types using Rust's
    buffer protocol access. It implements Python's iterator protocol and can
    be used in for loops, list comprehensions, and other iterator contexts.

    Examples:
        >>> import array
        >>> import arrayops as ao
        >>> arr = array.array('i', [1, 2, 3, 4, 5])
        >>> it = ao.array_iterator(arr)
        >>> list(it)
        [1, 2, 3, 4, 5]
        >>> it = ao.array_iterator(arr)
        >>> next(it)
        1
        >>> next(it)
        2
    """

    def __init__(self, source: Any) -> None:
        """
        Create a new ArrayIterator from a source array.

        Args:
            source: The source array to iterate over. Can be an array.array,
                numpy.ndarray, memoryview, or Arrow buffer/array.
        """
        ...

    def __iter__(self) -> "ArrayIterator":
        """
        Return the iterator itself (required by iterator protocol).

        Returns:
            ArrayIterator: The iterator object itself.
        """
        ...

    def __next__(self) -> Union[int, float]:
        """
        Return the next element in the iteration.

        Returns:
            Union[int, float]: The next element in the array.
                - Returns ``int`` for integer arrays
                - Returns ``float`` for float arrays

        Raises:
            StopIteration: When there are no more elements to iterate over.

        Examples:
            >>> import array
            >>> import arrayops as ao
            >>> arr = array.array('i', [1, 2, 3])
            >>> it = ao.array_iterator(arr)
            >>> next(it)
            1
            >>> next(it)
            2
            >>> next(it)
            3
            >>> next(it)
            Traceback (most recent call last):
                ...
            StopIteration
        """
        ...

def lazy_array(arr: _ArrayLike) -> "LazyArray":
    """
    Create a lazy array that can chain operations without intermediate allocations.

    This function creates a LazyArray object that allows chaining multiple
    operations (like map and filter) without creating intermediate arrays.
    Operations are deferred until ``collect()`` is called, making it more
    memory-efficient for complex operation chains.

    Args:
        arr: Input array with numeric type. Must be one of:
            - ``array.array`` with typecode: ``b``, ``B``, ``h``, ``H``, ``i``, ``I``, ``l``, ``L``, ``f``, ``d``
            - ``numpy.ndarray``: must be 1-dimensional and contiguous
            - ``memoryview``: read-only or writable memoryviews are supported
            - Apache Arrow buffers/arrays

    Returns:
        LazyArray: A lazy array object that supports chaining operations.
            Use methods like ``map()`` and ``filter()`` to chain operations,
            then call ``collect()`` to execute and get the result.

    Raises:
        TypeError: If input is not an ``array.array``, ``numpy.ndarray``, ``memoryview``, or Arrow buffer/array

    Notes:
        - Lazy arrays defer execution until ``collect()`` is called
        - Operations can be chained: ``lazy.map(fn).filter(pred).collect()``
        - More memory-efficient than multiple separate ``map()`` and ``filter()`` calls
        - Result is cached after first ``collect()`` call

    Examples:
        >>> import array
        >>> import arrayops as ao
        >>> arr = array.array('i', [1, 2, 3, 4, 5])
        >>> lazy = ao.lazy_array(arr)
        >>> result = lazy.map(lambda x: x * 2).filter(lambda x: x > 5).collect()
        >>> list(result)
        [6, 8, 10]
    """
    ...

class LazyArray:
    """
    A lazy array wrapper that chains operations without intermediate allocations.

    LazyArray allows you to chain multiple operations (like map and filter) on
    an array without creating intermediate arrays. Operations are deferred until
    ``collect()`` is called, making it more memory-efficient for complex
    operation chains.

    Examples:
        >>> import array
        >>> import arrayops as ao
        >>> arr = array.array('i', [1, 2, 3, 4, 5])
        >>> lazy = ao.lazy_array(arr)
        >>> result = lazy.map(lambda x: x * 2).filter(lambda x: x > 5).collect()
        >>> list(result)
        [6, 8, 10]
    """

    def __init__(self, source: Any) -> None:
        """
        Create a new LazyArray from a source array.

        Args:
            source: The source array to wrap. Can be an array.array, numpy.ndarray,
                memoryview, or Arrow buffer/array.
        """
        ...

    def map(self, function: Callable[[Any], Any]) -> "LazyArray":
        """
        Apply a function to each element (returns a new LazyArray, does not execute).

        This method adds a map operation to the lazy chain. The operation is not
        executed until ``collect()`` is called.

        Args:
            function: Callable function that takes a single numeric value and returns
                a numeric value. Can be a lambda, named function, or any callable object.

        Returns:
            LazyArray: New LazyArray with the map operation added to the chain.
                This allows for method chaining.

        Examples:
            >>> import array
            >>> import arrayops as ao
            >>> arr = array.array('i', [1, 2, 3])
            >>> lazy = ao.lazy_array(arr)
            >>> lazy = lazy.map(lambda x: x * 2)
            >>> result = lazy.collect()
            >>> list(result)
            [2, 4, 6]
        """
        ...

    def filter(self, predicate: Callable[[Any], bool]) -> "LazyArray":
        """
        Filter elements based on a predicate (returns a new LazyArray, does not execute).

        This method adds a filter operation to the lazy chain. The operation is not
        executed until ``collect()`` is called.

        Args:
            predicate: Callable function that takes a single numeric value and returns
                a boolean. Should return ``True`` for elements to keep, ``False`` for
                elements to filter out. Can be a lambda, named function, or any callable object.

        Returns:
            LazyArray: New LazyArray with the filter operation added to the chain.
                This allows for method chaining.

        Examples:
            >>> import array
            >>> import arrayops as ao
            >>> arr = array.array('i', [1, 2, 3, 4, 5])
            >>> lazy = ao.lazy_array(arr)
            >>> lazy = lazy.filter(lambda x: x > 2)
            >>> result = lazy.collect()
            >>> list(result)
            [3, 4, 5]
        """
        ...

    def collect(self) -> _ArrayLike:
        """
        Execute all chained operations and return the result.

        This method executes all operations in the lazy chain and returns the
        final result. The result is cached, so subsequent calls return the
        cached value without re-executing the operations.

        Returns:
            _ArrayLike: Result array with all operations applied.
                - Returns ``numpy.ndarray`` if input was ``numpy.ndarray`` or Arrow array
                - Returns ``array.array`` if input was ``array.array`` or ``memoryview``
                - Result type matches original input type

        Examples:
            >>> import array
            >>> import arrayops as ao
            >>> arr = array.array('i', [1, 2, 3, 4, 5])
            >>> lazy = ao.lazy_array(arr)
            >>> lazy = lazy.map(lambda x: x * 2).filter(lambda x: x > 5)
            >>> result = lazy.collect()
            >>> list(result)
            [6, 8, 10]
        """
        ...

    def source(self) -> _ArrayLike:
        """
        Get the source array.

        This method returns the original source array that was used to create
        the LazyArray.

        Returns:
            _ArrayLike: The original source array. Type matches the input type
                (array.array, numpy.ndarray, memoryview, or Arrow buffer/array).

        Examples:
            >>> import array
            >>> import arrayops as ao
            >>> arr = array.array('i', [1, 2, 3, 4, 5])
            >>> lazy = ao.lazy_array(arr)
            >>> source = lazy.source()
            >>> list(source)
            [1, 2, 3, 4, 5]
        """
        ...

    def len(self) -> int:
        """
        Get the number of operations in the chain.

        This method returns the number of operations (map, filter, etc.) that
        have been added to the lazy chain.

        Returns:
            int: The number of operations in the chain. Returns 0 if no
                operations have been added yet.

        Examples:
            >>> import array
            >>> import arrayops as ao
            >>> arr = array.array('i', [1, 2, 3])
            >>> lazy = ao.lazy_array(arr)
            >>> lazy.len()
            0
            >>> lazy = lazy.map(lambda x: x * 2)
            >>> lazy.len()
            1
            >>> lazy = lazy.filter(lambda x: x > 2)
            >>> lazy.len()
            2
        """
        ...

    def __iter__(self) -> "ArrayIterator":
        """
        Iterator protocol: evaluate lazy chain and return an iterator.

        This method evaluates the lazy chain (if not already cached) and returns
        an ArrayIterator over the result. This enables using LazyArray in for
        loops and other iterator contexts.

        Returns:
            ArrayIterator: An iterator over the evaluated array result.

        Examples:
            >>> import array
            >>> import arrayops as ao
            >>> arr = array.array('i', [1, 2, 3, 4, 5])
            >>> lazy = ao.lazy_array(arr)
            >>> lazy = lazy.map(lambda x: x * 2).filter(lambda x: x > 5)
            >>> for x in lazy:
            ...     print(x)
            6
            8
            10
        """
        ...
