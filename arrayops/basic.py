"""Basic operations for arrayops.

This module provides fundamental array operations:
- sum: Compute sum of elements
- scale: Scale elements in-place
- mean: Compute arithmetic mean
- min: Find minimum value
- max: Find maximum value
"""

from arrayops._arrayops import max, mean, min, scale, sum  # noqa: F401

__all__ = ["sum", "scale", "mean", "min", "max"]
