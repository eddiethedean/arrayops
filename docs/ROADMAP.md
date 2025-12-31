# Roadmap

This document outlines the planned features and improvements for the `arrayops` project.

## ✅ Completed

### Core Operations
- [x] **Sum operation** - Fast summation for all numeric array types
- [x] **Scale operation** - In-place scaling with type-safe factor application
- [x] **Full test coverage** - 100% coverage for both Python and Rust code
- [x] **Type stubs for mypy** - Complete type annotations for static type checking

### Phase 1: Additional Operations
- [x] **`map(arr, fn) -> array`** - Apply function to each element, return new array
  - Support for Python callables (lambda, functions)
  - Type preservation (input type determines output type)
  - Performance target: 10-50x faster than Python list comprehension

- [x] **`map_inplace(arr, fn) -> None`** - Apply function in-place
  - Modify array elements without allocation
  - Same callable support as `map`
  - Performance target: 5-20x faster than Python loop

- [x] **`filter(arr, predicate) -> array`** - Return new array with filtered elements
  - Support for Python callable predicates
  - Preserve original array type
  - Handle empty results gracefully
  - Performance target: 10-30x faster than Python list comprehension

- [x] **`reduce(arr, fn, initial=None) -> scalar`** - Fold array with binary function
  - Support for Python callables
  - Optional initial value
  - Type inference for return value
  - Performance target: 10-40x faster than Python `functools.reduce`

### Phase 2: Performance Optimizations
- [x] **Parallel execution (Rayon)** - Parallel processing for large arrays
  - Feature flag: `parallel` - Enable with `--features parallel` or `maturin build --features parallel`
  - Thread-safe buffer access via Vec extraction
  - Threshold-based parallelization (10,000 elements for sum/reduce, 5,000 for scale)
  - Implemented for: sum, scale operations
  - Performance: Near-linear speedup on multi-core systems

- [x] **SIMD infrastructure** - Framework for SIMD optimizations
  - Feature flag: `simd` - Enable with `--features simd`
  - Infrastructure in place for future SIMD implementation
  - Note: Full SIMD implementation pending std::simd API stabilization
  - When implemented: 2-4x additional speedup expected

## 🚧 In Progress

_No items currently in progress_

## 📋 Planned Features

### Phase 3: Interoperability (Medium Priority)
- [x] **NumPy array support** - Operate on `numpy.ndarray` objects
  - Zero-copy access via NumPy's buffer protocol
  - Support for contiguous 1D arrays
  - Type conversion handling
  - Performance: Match or exceed NumPy's built-in operations
  - Optional dependency (only import when NumPy available)
  - Returns `numpy.ndarray` for `map` and `filter` operations

- [x] **Python memoryview support** - Work with `memoryview` objects
  - Buffer protocol access
  - Type inference from memoryview format
  - Support for both read-only and writable memoryviews
  - Use case: Binary data processing, network protocols
  - In-place operations require writable memoryviews

### Phase 4: Advanced Features (Low Priority)

#### Statistical Operations
- [ ] **`mean(arr) -> float`** - Arithmetic mean
- [ ] **`min(arr) -> scalar`** - Minimum value
- [ ] **`max(arr) -> scalar`** - Maximum value
- [ ] **`std(arr) -> float`** - Standard deviation
- [ ] **`var(arr) -> float`** - Variance
- [ ] **`median(arr) -> scalar`** - Median value

#### Element-wise Operations
- [ ] **`add(arr1, arr2) -> array`** - Element-wise addition
- [ ] **`multiply(arr1, arr2) -> array`** - Element-wise multiplication
- [ ] **`clip(arr, min, max) -> None`** - In-place clipping to range
- [ ] **`normalize(arr) -> None`** - In-place normalization to [0, 1]

#### Array Manipulation
- [ ] **`reverse(arr) -> None`** - In-place reversal
- [ ] **`sort(arr) -> None`** - In-place sorting (for numeric types)
- [ ] **`unique(arr) -> array`** - Return unique elements (sorted)

## 🔬 Research & Exploration

### Potential Enhancements
- [ ] **Arrow buffer interop** - Support Apache Arrow memory format
- [ ] **Zero-copy slicing** - Return views instead of copies where possible
- [ ] **Lazy evaluation** - Chain operations without intermediate allocations
- [ ] **Custom allocators** - Support for specialized memory pools
- [ ] **GPU acceleration** - Optional CUDA/OpenCL support for very large arrays

### API Design Considerations
- [ ] **Method chaining** - Consider fluent API: `arr.map(fn).filter(pred).sum()`
- [ ] **Iterator protocol** - Support Python iteration efficiently
- [ ] **Context managers** - Resource management for parallel operations
- [ ] **Async support** - Async/await for I/O-bound operations

## 📊 Success Metrics

### Performance Targets
- **Sum**: 100x faster than Python (✅ achieved)
- **Scale**: 50x faster than Python (✅ achieved)
- **Map**: 10-50x faster than list comprehension
- **Filter**: 10-30x faster than list comprehension
- **Parallel ops**: Near-linear speedup on 4-8 core systems

### Quality Targets
- Maintain 100% test coverage
- Zero memory safety issues
- Full mypy type checking compliance
- Comprehensive documentation with examples

## 🗓️ Timeline

### Q1 2024
- [x] Complete Phase 1 (map, filter, reduce operations)
- [x] Complete Phase 2 (parallel execution infrastructure, SIMD framework)

### Q2 2024
- [x] Implement parallel execution with rayon (completed in Q1 2024)
- [ ] Complete full SIMD optimizations (infrastructure in place, pending API stabilization)
- [x] NumPy interop (completed in Q1 2024)
- [x] Memoryview support (completed in Q1 2024)

### Q3 2024
- [x] Complete NumPy and memoryview support (completed in Q1 2024)
- [ ] Statistical operations
- [ ] Performance benchmarking suite

### Q4 2024
- [ ] Advanced features (element-wise ops, array manipulation)
- [ ] API polish and documentation
- [ ] Version 1.0 release candidate

### Q1 2025
- [ ] **Version 1.0 release** - Official stable release
- [ ] Post-release bug fixes and stability improvements
- [ ] Performance profiling and optimization pass
- [ ] Community feedback integration
- [ ] Begin Arrow buffer interop research
- [ ] Zero-copy slicing implementation
- [ ] Enhanced error messages and diagnostics

### Q2 2025
- [ ] **Arrow buffer interop** - Support Apache Arrow memory format
- [ ] Iterator protocol optimization for efficient Python iteration
- [ ] Method chaining API design and prototype
- [ ] Advanced SIMD optimizations (platform-specific tuning)
- [ ] Performance regression testing infrastructure
- [ ] Documentation site and tutorials
- [ ] Community benchmarks and case studies

### Q3 2025
- [ ] **Lazy evaluation system** - Chain operations without intermediate allocations
- [ ] Custom allocator support for specialized memory pools
- [ ] Async/await support for I/O-bound operations
- [ ] Context managers for parallel operation resource management
- [ ] Extended statistical operations (percentiles, quantiles)
- [ ] Multi-dimensional array support research (if demand exists)
- [ ] Ecosystem integration (pandas, polars compatibility layers)

### Q4 2025
- [ ] **GPU acceleration research** - CUDA/OpenCL feasibility study
- [ ] Advanced array manipulation (reshape, transpose concepts)
- [ ] Streaming/chunked processing for very large arrays
- [ ] Memory-mapped file support
- [ ] Performance optimization pass based on real-world usage
- [ ] Version 2.0 planning and design
- [ ] Community workshop and conference presentations

## 🤝 Contributing to the Roadmap

We welcome contributions! If you're interested in implementing any of these features:

1. Check existing issues and pull requests
2. Open an issue to discuss the approach
3. Follow the contribution guidelines in README.md
4. Ensure 100% test coverage
5. Update documentation

Priority will be given to:
- Features with clear use cases
- Performance improvements
- Interoperability enhancements
- Bug fixes and stability improvements

## 📝 Notes

- All features should maintain backward compatibility
- Performance is a primary concern - new features should not regress existing operations
- Type safety is critical - all operations must validate inputs
- Documentation and examples are required for all new features

---

_Last updated: Phase 3 completed (NumPy array interop and memoryview support)_

