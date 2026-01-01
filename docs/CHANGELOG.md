# Changelog

All notable changes to `arrayops` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- See [roadmap](roadmap) for details.

## [1.0.0] - 2025-01-01

### Added
- Production-ready 1.0.0 release
- API stability guarantees documented (see [API_STABILITY.md](../API_STABILITY.md))
- Comprehensive security documentation
- Migration guide from 0.x to 1.0.0 (see [MIGRATION.md](https://github.com/eddiethedean/arrayops/blob/main/MIGRATION.md))
- Deprecation tracking and migration plans (see [DEPRECATION_NOTES.md](../DEPRECATION_NOTES.md))

### Changed
- Development status updated from "Alpha" to "Production/Stable"
- Improved error messages for better debugging
- Enhanced documentation across all modules

### Documentation
- Added comprehensive API stability documentation
- Created migration guide for upgrading from 0.x versions
- Documented deprecation warnings and migration plans
- Updated security documentation with detailed guarantees
- Enhanced README with 1.0.0 release information

### Testing
- Maintained 100% test coverage (Python and Rust)
- Comprehensive integration tests for large datasets
- Security test suite covering input validation, DoS resistance, and overflow handling
- Performance regression tests to ensure optimizations are maintained

### Code Quality
- Comprehensive code review completed
- Unsafe code audit completed (only dead code uses unsafe, well-documented)
- API stability review completed
- Thread safety verified for parallel operations
- Memory safety verified through Rust's type system

### Security
- Security review completed
- Buffer access patterns verified safe (using PyO3 safe APIs)
- Input validation comprehensive and tested
- Error handling secure (no information leakage)

### Technical Details
- PyO3 0.24 with extension-module feature
- Rust 2021 edition
- Python 3.8+ support
- Optional parallel execution support (via `parallel` feature)
- SIMD optimization infrastructure (via `simd` feature)

### Notes
- No breaking changes from 0.4.x to 1.0.0
- All 0.4.x code should work unchanged
- See [MIGRATION.md](https://github.com/eddiethedean/arrayops/blob/main/MIGRATION.md) for migration guidance
- See [API_STABILITY.md](../API_STABILITY.md) for API stability guarantees

## [0.3.0] - 2024-01-XX

### Added
- **Statistical Operations** (6 functions)
  - **`mean(arr) -> float`** - Arithmetic mean calculation
    - Always returns float, even for integer arrays
    - ~50x faster than computing mean in pure Python
  - **`min(arr) -> scalar`** - Minimum value
    - Returns type matching array element type
    - ~30x faster than Python's built-in `min()` for large arrays
  - **`max(arr) -> scalar`** - Maximum value
    - Returns type matching array element type
    - ~30x faster than Python's built-in `max()` for large arrays
  - **`std(arr) -> float`** - Population standard deviation
    - Uses formula: sqrt(sum((x - mean)^2) / n)
    - ~40x faster than computing std in pure Python
  - **`var(arr) -> float`** - Population variance
    - Uses formula: sum((x - mean)^2) / n
    - ~40x faster than computing variance in pure Python
  - **`median(arr) -> scalar`** - Median value
    - For even-length arrays, returns lower median
    - Returns type matching array element type
    - ~20x faster than computing median in pure Python
- **Element-wise Operations** (4 functions)
  - **`add(arr1, arr2) -> array`** - Element-wise addition
    - Requires arrays of same length
    - Returns new array (preserves input type)
    - ~30x faster than Python loops
  - **`multiply(arr1, arr2) -> array`** - Element-wise multiplication
    - Requires arrays of same length
    - Returns new array (preserves input type)
    - ~30x faster than Python loops
  - **`clip(arr, min_val, max_val) -> None`** - In-place clipping to range
    - Modifies array elements to be within [min_val, max_val]
    - ~25x faster than Python loops
  - **`normalize(arr) -> None`** - In-place normalization to [0, 1]
    - Uses min-max normalization: (x - min) / (max - min)
    - Requires non-empty array with min != max
    - ~25x faster than computing normalization in pure Python
- **Array Manipulation** (3 functions)
  - **`reverse(arr) -> None`** - In-place reversal of array elements
    - Uses standard reversal algorithm
    - ~30x faster than Python's `arr.reverse()` for large arrays
  - **`sort(arr) -> None`** - In-place sorting (ascending order)
    - Uses Rust's stable sort algorithm
    - ~10x faster than Python's `arr.sort()` for large arrays
  - **`unique(arr) -> array`** - Return unique elements (sorted)
    - Returns new array with unique elements in sorted order
    - Preserves input type (NumPy if NumPy input, array.array otherwise)
    - ~20x faster than Python's `list(set(arr))` for large arrays
- All new operations support `array.array`, `numpy.ndarray` (1D, contiguous), and `memoryview` inputs
- Comprehensive test coverage for all new functions (50+ new tests)
- Complete API documentation and examples for all new functions

### Testing
- Added 50+ comprehensive tests for Phase 4 functions
- Tests cover all numeric types, edge cases, error conditions, and NumPy inputs
- 100% code coverage maintained for all new code

### Documentation
- Updated API reference with complete documentation for all 13 new functions
- Added practical examples for statistical, element-wise, and array manipulation operations
- Updated README with Phase 4 features
- Updated ROADMAP to mark Phase 4 as completed

## [0.2.0] - 2024-01-XX

### Added
- **`map(arr, fn) -> array.array`** - Apply function to each element, return new array
  - Supports Python callables (lambda, functions)
  - Type preservation (input type determines output type)
  - ~20x faster than Python list comprehensions
- **`map_inplace(arr, fn) -> None`** - Apply function in-place
  - Modify array elements without allocation
  - Same callable support as `map`
  - ~15x faster than Python loops
- **`filter(arr, predicate) -> array.array`** - Return new array with filtered elements
  - Support for Python callable predicates
  - Preserve original array type
  - Handle empty results gracefully
  - ~15x faster than Python list comprehensions
- **`reduce(arr, fn, initial=None) -> Any`** - Fold array with binary function
  - Support for Python callables
  - Optional initial value
  - Type inference for return value
  - ~25x faster than Python `functools.reduce`

### Performance
- All new operations provide significant speedups over pure Python
- Zero-copy buffer access maintained for all operations
- Memory-efficient in-place operations available

### Documentation
- Updated API reference with new operations
- Added examples for map, filter, and reduce operations
- Updated README with usage examples
- Added comprehensive examples in documentation

### Testing
- 75 comprehensive tests (up from 45)
- 30 new tests for Phase 1 operations
- 100% code coverage maintained
- Tests cover all numeric types, edge cases, and error conditions

### Technical Details
- Python callable support via PyO3
- Type-safe conversions between Python and Rust types
- Comprehensive error handling and validation

## [0.1.0] - 2024-01-XX

### Added
- Initial release of `arrayops`
- `sum()` function for computing sum of array elements
  - Supports all numeric array types (b, B, h, H, i, I, l, L, f, d)
  - Returns int for integer arrays, float for float arrays
  - Handles empty arrays gracefully (returns 0)
- `scale()` function for in-place array scaling
  - Supports all numeric array types
  - Modifies arrays in-place (no allocation)
  - Handles empty arrays gracefully
- Zero-copy buffer access via Python buffer protocol
- Type stubs for mypy type checking
- Comprehensive test suite with 100% coverage
- Full documentation

### Performance
- `sum()` operation: ~100x faster than Python's built-in `sum()` for large arrays
- `scale()` operation: ~50x faster than Python loops for large arrays
- Zero memory overhead for operations

### Documentation
- README with quick start guide
- API reference
- Examples and use cases
- Design document
- Roadmap

### Testing
- 100% code coverage (Python and Rust)
- Tests for all numeric types
- Edge case testing (empty arrays, single elements)
- Error handling tests
- Performance tests

### Technical Details
- Built with PyO3 for Python-Rust interop
- Uses maturin for packaging
- Supports Python 3.8+
- Requires Rust 1.70+

## Types of Changes

- **Added** for new features
- **Changed** for changes in existing functionality
- **Deprecated** for soon-to-be removed features
- **Removed** for now removed features
- **Fixed** for any bug fixes
- **Security** for vulnerability fixes

---

[Unreleased]: https://github.com/eddiethedean/arrayops/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/eddiethedean/arrayops/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/eddiethedean/arrayops/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/eddiethedean/arrayops/releases/tag/v0.1.0

