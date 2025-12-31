"""arrayops - Rust-backed acceleration for Python array.array

This package provides fast, Rust-accelerated operations for Python's built-in
array.array type, supporting numeric operations like sum and scale.
"""

__version__ = "0.2.0"

try:
    from arrayops._arrayops import sum, scale, map, map_inplace, filter, reduce

    __all__ = ["sum", "scale", "map", "map_inplace", "filter", "reduce"]
except ImportError as e:
    # Module not yet built - provide helpful error message
    if "_arrayops" in str(e):
        raise ImportError(
            "arrayops extension module not found. "
            "Build the package with 'maturin develop' or 'pip install -e .'"
        ) from e
    raise
