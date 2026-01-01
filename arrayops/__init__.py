"""arrayops - Rust-backed acceleration for Python array.array

This package provides fast, Rust-accelerated operations for Python's built-in
array.array type, supporting a wide range of numeric operations including:

**Basic Operations:**
  - ``sum()`` - Compute sum of elements
  - ``scale()`` - Scale elements in-place
  - ``mean()`` - Compute arithmetic mean
  - ``min()``, ``max()`` - Find minimum/maximum values

**Transformations:**
  - ``map()``, ``map_inplace()`` - Apply function to each element
  - ``filter()`` - Filter elements based on predicate
  - ``reduce()`` - Fold array with binary function

**Statistical Operations:**
  - ``std()``, ``var()`` - Standard deviation and variance
  - ``median()`` - Find median value

**Element-wise Operations:**
  - ``add()``, ``multiply()`` - Element-wise arithmetic
  - ``clip()`` - Clip values to range
  - ``normalize()`` - Normalize to [0, 1] range

**Array Manipulation:**
  - ``reverse()`` - Reverse array in-place
  - ``sort()`` - Sort array in-place
  - ``unique()`` - Get unique elements

**Advanced Features:**
  - ``slice()`` - Zero-copy array slicing
  - ``lazy_array()`` - Lazy evaluation for operation chaining
  - Support for ``numpy.ndarray``, ``memoryview``, and Apache Arrow buffers

**Performance:**
  - Operations are significantly faster than pure Python implementations
  - Optional parallel execution for multi-core systems (``--features parallel``)
  - SIMD optimization infrastructure (``--features simd``)

**Supported Types:**
  - Input: ``array.array``, ``numpy.ndarray``, ``memoryview``, Apache Arrow buffers/arrays
  - Typecodes: ``b``, ``B``, ``h``, ``H``, ``i``, ``I``, ``l``, ``L``, ``f``, ``d``

Example:
    >>> import array
    >>> import arrayops as ao
    >>> arr = array.array('i', [1, 2, 3, 4, 5])
    >>> ao.sum(arr)
    15
    >>> arr2 = array.array('i', [1, 2, 3, 4, 5])
    >>> ao.scale(arr2, 2.0)
    >>> list(arr2)
    [2, 4, 6, 8, 10]
"""

__version__ = "1.0.0"

# Import from organized submodules
try:
    from arrayops.basic import max, mean, min, scale, sum
    from arrayops.transform import filter, map, map_inplace, reduce
    from arrayops.stats import median, std, var
    from arrayops.stats import std_dev  # noqa: F401  # Backward compatibility alias
    from arrayops.elementwise import add, clip, multiply, normalize
    from arrayops.manipulation import reverse, sort, unique
    from arrayops.slice import slice
    from arrayops.iterator import ArrayIterator, array_iterator
    from arrayops.lazy import LazyArray, lazy_array

    __all__ = [
        # Basic operations
        "sum",
        "scale",
        "mean",
        "min",
        "max",
        # Transform operations
        "map",
        "map_inplace",
        "filter",
        "reduce",
        # Statistical operations
        "std",
        "var",
        "median",
        # Element-wise operations
        "add",
        "multiply",
        "clip",
        "normalize",
        # Manipulation operations
        "reverse",
        "sort",
        "unique",
        # Slice operations
        "slice",
        # Iterator
        "array_iterator",
        "ArrayIterator",
        # Lazy evaluation
        "lazy_array",
        "LazyArray",
    ]
except ImportError as e:
    # Module not yet built - provide helpful error message
    if "_arrayops" in str(e):
        raise ImportError(
            "arrayops extension module not found. "
            "Build the package with 'maturin develop' or 'pip install -e .'"
        ) from e
    raise
