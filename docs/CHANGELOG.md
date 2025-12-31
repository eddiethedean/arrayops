# Changelog

All notable changes to `arrayops` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Map, filter, and reduce operations
- Parallel execution support (rayon)
- SIMD auto-vectorization
- NumPy array interop
- Memoryview support

See [ROADMAP.md](ROADMAP.md) for details.

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

[Unreleased]: https://github.com/your-username/arrayops/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/your-username/arrayops/releases/tag/v0.1.0

