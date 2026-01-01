"""Iterator functionality for arrayops.

This module provides iterator support:
- array_iterator: Create an iterator over array elements
- ArrayIterator: Iterator class for array elements
"""

from arrayops._arrayops import ArrayIterator, array_iterator  # noqa: F401

__all__ = ["array_iterator", "ArrayIterator"]
