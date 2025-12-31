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

__version__ = "0.4.0"

try:
    from arrayops._arrayops import (
        sum,
        scale,
        map,
        map_inplace,
        filter,
        reduce,
        mean,
        min,
        max,
        std,
        var,
        median,
        add,
        multiply,
        clip,
        normalize,
        reverse,
        sort,
        unique,
        slice,
        lazy_array,
        LazyArray,
    )

    __all__ = [
        "sum",
        "scale",
        "map",
        "map_inplace",
        "filter",
        "reduce",
        "mean",
        "min",
        "max",
        "std",
        "var",
        "median",
        "add",
        "multiply",
        "clip",
        "normalize",
        "reverse",
        "sort",
        "unique",
        "slice",
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
