"""Type stubs for arrayops._arrayops Rust extension module."""

from typing import Any, Union

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

