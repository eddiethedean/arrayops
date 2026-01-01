"""Statistical operations for arrayops.

This module provides statistical operations:
- var: Compute variance
- std: Compute standard deviation
- std_dev: Alias for std (backward compatibility)
- median: Find median value
"""

from arrayops._arrayops import median, std, var  # noqa: F401

# Alias std_dev to std for backward compatibility
std_dev = std  # noqa: F401

__all__ = ["var", "std", "std_dev", "median"]
