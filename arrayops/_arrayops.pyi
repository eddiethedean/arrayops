"""Type stubs for arrayops._arrayops Rust extension module."""

from typing import Any, Callable, Optional, Union

def sum(arr: Any) -> Union[int, float]:
    """
    Compute the sum of all elements in an array.

    Args:
        arr: Input array.array (numeric types only)

    Returns:
        Integer for integer arrays, float for float arrays
    """
    ...

def scale(arr: Any, factor: float) -> None:
    """
    Scale all elements of an array in-place by a factor.

    Args:
        arr: Input array.array (numeric types only)
        factor: Scaling factor
    """
    ...

def map(arr: Any, fn: Callable[[Union[int, float]], Union[int, float]]) -> Any:
    """
    Apply function to each element, return new array.

    Args:
        arr: Input array.array (numeric types only)
        fn: Function to apply to each element

    Returns:
        New array.array with mapped values
    """
    ...

def map_inplace(arr: Any, fn: Callable[[Union[int, float]], Union[int, float]]) -> None:
    """
    Apply function to each element in-place.

    Args:
        arr: Input array.array (numeric types only)
        fn: Function to apply to each element
    """
    ...

def filter(arr: Any, predicate: Callable[[Union[int, float]], bool]) -> Any:
    """
    Return new array with filtered elements.

    Args:
        arr: Input array.array (numeric types only)
        predicate: Predicate function that returns True for elements to keep

    Returns:
        New array.array with filtered elements
    """
    ...

def reduce(
    arr: Any, fn: Callable[[Any, Union[int, float]], Any], initial: Optional[Any] = None
) -> Any:
    """
    Fold array with binary function.

    Args:
        arr: Input array.array (numeric types only)
        fn: Binary function to accumulate values
        initial: Optional initial value. If None and array is empty, raises ValueError

    Returns:
        Accumulated value (type depends on function and initial value)
    """
    ...
