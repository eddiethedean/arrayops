"""Lazy evaluation for arrayops.

This module provides lazy evaluation functionality:
- lazy_array: Create a lazy array wrapper
- LazyArray: Lazy array class for operation chaining
"""

from arrayops._arrayops import LazyArray, lazy_array  # noqa: F401

__all__ = ["lazy_array", "LazyArray"]
