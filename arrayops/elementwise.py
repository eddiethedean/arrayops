"""Element-wise operations for arrayops.

This module provides element-wise array operations:
- add: Element-wise addition
- multiply: Element-wise multiplication
- clip: Clip values to range
- normalize: Normalize to [0, 1] range
"""

from arrayops._arrayops import add, clip, multiply, normalize  # noqa: F401

__all__ = ["add", "multiply", "clip", "normalize"]
