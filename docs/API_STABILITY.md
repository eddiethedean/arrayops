# API Stability Guarantees

This document describes the API stability guarantees for arrayops 1.0.0 and beyond.

## Version 1.0.0 API Stability

As of version 1.0.0, arrayops follows [Semantic Versioning](https://semver.org/):
- **MAJOR version** (1.x.x): Breaking API changes
- **MINOR version** (x.1.x): New functionality in a backward-compatible manner
- **PATCH version** (x.x.1): Backward-compatible bug fixes

## Public API Surface

The public API consists of:

### Functions

All functions are exported from the `arrayops` package and its submodules:

**Basic Operations:**
- `sum(arr)` - Compute sum of elements
- `scale(arr, factor)` - Scale elements in-place
- `mean(arr)` - Compute arithmetic mean
- `min(arr)` - Find minimum value
- `max(arr)` - Find maximum value

**Transform Operations:**
- `map(arr, fn)` - Apply function to each element
- `map_inplace(arr, fn)` - Apply function to each element in-place
- `filter(arr, predicate)` - Filter elements based on predicate
- `reduce(arr, fn, initial=None)` - Fold array with binary function

**Statistical Operations:**
- `std(arr)` - Population standard deviation
- `var(arr)` - Population variance
- `median(arr)` - Find median value

**Element-wise Operations:**
- `add(arr1, arr2)` - Element-wise addition
- `multiply(arr1, arr2)` - Element-wise multiplication
- `clip(arr, min_val, max_val)` - Clip values to range (in-place)
- `normalize(arr)` - Normalize to [0, 1] range (in-place)

**Manipulation Operations:**
- `reverse(arr)` - Reverse array in-place
- `sort(arr)` - Sort array in-place
- `unique(arr)` - Get unique elements

**Slice Operations:**
- `slice(arr, start=None, end=None)` - Zero-copy array slicing

**Iterator:**
- `array_iterator(arr)` - Create iterator over array elements
- `ArrayIterator` - Iterator class

**Lazy Evaluation:**
- `lazy_array(arr)` - Create lazy array wrapper
- `LazyArray` - Lazy array class

### Classes

- `ArrayIterator` - Iterator for array elements
- `LazyArray` - Lazy evaluation wrapper for operation chaining

## API Stability Guarantees

### Guaranteed Stable (1.0.0+)

1. **Function Signatures**: All public function signatures are stable
   - Parameter names and types will not change
   - Return types will not change
   - Optional parameters will remain optional

2. **Function Behavior**: Core behavior is stable
   - Operations produce consistent results
   - Error conditions remain the same
   - Performance characteristics are maintained or improved

3. **Input Types**: Supported input types are stable
   - `array.array` - Primary supported type
   - `numpy.ndarray` (1D, contiguous) - Supported
   - `memoryview` - Supported
   - Apache Arrow buffers/arrays - Supported

4. **Typecodes**: Supported typecodes are stable
   - Signed integers: `b`, `h`, `i`, `l`
   - Unsigned integers: `B`, `H`, `I`, `L`
   - Floats: `f`, `d`

### Future Changes

The following may change in MINOR versions (new functionality):

1. **New Functions**: New operations may be added
2. **New Classes**: New utility classes may be added
3. **Performance Improvements**: Performance may improve without API changes
4. **New Features**: New optional features may be added

The following require MAJOR versions (breaking changes):

1. **Removed Functions**: Functions will not be removed without a major version bump
2. **Changed Signatures**: Function signatures will not change without a major version bump
3. **Changed Behavior**: Breaking behavior changes require a major version bump
4. **Removed Support**: Removing support for input types or typecodes requires a major version bump

## Known Limitations

### API Limitations

1. **NumPy Arrays**: Only 1D, contiguous NumPy arrays are supported
   - Multi-dimensional arrays: Not supported
   - Non-contiguous arrays: Not supported (will raise ValueError)

2. **Memoryview**: Read-only memoryviews cannot be used with in-place operations
   - Operations like `scale()`, `clip()`, `normalize()`, `reverse()`, `sort()` require writable memoryviews

3. **Arrow Buffers**: Immutable - in-place operations are not supported
   - Operations like `scale()`, `clip()`, `normalize()` will raise ValueError

4. **Type Restrictions**: Only numeric types are supported
   - Object arrays (e.g., `array.array('O', [...])`): Not supported
   - String arrays: Not supported
   - Mixed types: Not supported

5. **Empty Arrays**: Some operations handle empty arrays specially:
   - `sum()`: Returns 0 (or 0.0 for floats)
   - `mean()`: Raises ValueError (cannot compute mean of empty array)
   - `std()`, `var()`: Raise ValueError
   - `min()`, `max()`: Raise ValueError

6. **Integer Overflow**: Integer arithmetic may wrap (Rust default behavior)
   - Python integers are arbitrary precision, but array.array uses fixed-size integers
   - Overflow behavior matches Rust's default (wrapping arithmetic)
   - Consider using floats for large numbers that may overflow

### Error Handling

1. **Error Types**: Consistent error types are used:
   - `TypeError`: Wrong input type or unsupported typecode
   - `ValueError`: Invalid values (e.g., empty array, invalid slice indices)
   - `BufferError`: Buffer access issues

2. **Error Messages**: Error messages may be improved in patch versions
   - Core error conditions remain the same
   - Messages may become more descriptive

## Migration Considerations

### From 0.x to 1.0.0

See [MIGRATION.md](MIGRATION.md) for detailed migration guide.

Key points:
- No breaking changes from 0.4.x to 1.0.0
- All 0.4.x code should work unchanged
- New features are additive only

### Future Versions

When migrating to future versions:
- MINOR versions (1.1.0, 1.2.0, etc.): No changes required
- PATCH versions (1.0.1, 1.0.2, etc.): No changes required
- MAJOR versions (2.0.0, etc.): Check migration guide for breaking changes

## Stability Notes

- **Internal Implementation**: Internal implementation details may change
- **Private APIs**: Private functions and modules are not stable
- **Performance**: Performance may improve or change (usually improves)
- **Dependencies**: Dependency versions may change (backward compatible versions only)

## Contact

For questions about API stability or migration:
- GitHub Issues: Report concerns or questions
- Documentation: Check migration guides and changelog
- Security: See [SECURITY.md](../SECURITY.md) for security-related API considerations

