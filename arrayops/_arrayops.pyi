"""Type stubs for arrayops._arrayops Rust extension module."""

from typing import Any, Callable, Optional, Union

try:
    import numpy as np
except ImportError:
    # NumPy may not be installed - define a stub type
    class np:
        class ndarray: ...

# Supported input types: array.array, numpy.ndarray, or memoryview
_ArrayLike = Union["array.array", "np.ndarray", memoryview]

def sum(arr: _ArrayLike) -> Union[int, float]:
    """
    Compute the sum of all elements in an array.

    Args:
        arr: Input array.array, numpy.ndarray (1D, contiguous), or memoryview (numeric types only)

    Returns:
        Integer for integer arrays, float for float arrays
    """
    ...

def scale(arr: _ArrayLike, factor: float) -> None:
    """
    Scale all elements of an array in-place by a factor.

    Args:
        arr: Input array.array, numpy.ndarray (1D, contiguous), or memoryview (numeric types only)
            For memoryview: must be writable
        factor: Scaling factor
    """
    ...

def map(
    arr: _ArrayLike, fn: Callable[[Union[int, float]], Union[int, float]]
) -> Union["array.array", "np.ndarray"]:
    """
    Apply function to each element, return new array.

    Args:
        arr: Input array.array, numpy.ndarray (1D, contiguous), or memoryview (numeric types only)
        fn: Function to apply to each element

    Returns:
        New array.array or numpy.ndarray with mapped values
        Returns numpy.ndarray if input is numpy.ndarray, otherwise array.array
    """
    ...

def map_inplace(
    arr: _ArrayLike, fn: Callable[[Union[int, float]], Union[int, float]]
) -> None:
    """
    Apply function to each element in-place.

    Args:
        arr: Input array.array, numpy.ndarray (1D, contiguous), or memoryview (numeric types only)
            For memoryview: must be writable
        fn: Function to apply to each element
    """
    ...

def filter(
    arr: _ArrayLike, predicate: Callable[[Union[int, float]], bool]
) -> Union["array.array", "np.ndarray"]:
    """
    Return new array with filtered elements.

    Args:
        arr: Input array.array, numpy.ndarray (1D, contiguous), or memoryview (numeric types only)
        predicate: Predicate function that returns True for elements to keep

    Returns:
        New array.array or numpy.ndarray with filtered elements
        Returns numpy.ndarray if input is numpy.ndarray, otherwise array.array
    """
    ...

def reduce(
    arr: _ArrayLike, fn: Callable[[Any, Union[int, float]], Any], initial: Optional[Any] = None
) -> Any:
    """
    Fold array with binary function.

    Args:
        arr: Input array.array, numpy.ndarray (1D, contiguous), or memoryview (numeric types only)
        fn: Binary function to accumulate values
        initial: Optional initial value. If None and array is empty, raises ValueError

    Returns:
        Accumulated value (type depends on function and initial value)
    """
    ...
